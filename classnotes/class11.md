# Class 11 — Async Rust & Tokio Deep Dive

## What we did

- Created `todo-api` project, added axum + tokio dependencies
- Ran a minimal axum Hello World server
- Deep dive: Futures, Tokio internals, async vs threads, when not to use async

---

## The Hello World server

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
axum = "0.8"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

```rust
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> &'static str {
    "Hello, world!"
}
```

Test with: `curl http://localhost:3000`

---

## Axum vs Tokio — roles

| | Role | Analogy |
|---|---|---|
| **Tokio** | The engine — runs async tasks, manages threads, drives I/O | Node.js event loop |
| **Axum** | The framework — routing, parsing requests, calling handlers | Express.js |

Axum knows *what* to do with HTTP requests. Tokio provides the machinery to *run* async code and handle many connections concurrently.

`#[tokio::main]` bootstraps the runtime. Without it, you can't `.await` anything. It expands roughly to:

```rust
fn main() {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            // your async main body
        });
}
```

---

## Where else can you use Tokio?

Tokio is general-purpose async I/O — anywhere you're waiting on something external:

- **HTTP clients** — concurrent API calls (`reqwest`)
- **Database access** — async queries (`sqlx`)
- **File I/O** — `tokio::fs`
- **TCP/UDP servers** — raw sockets, game servers, custom protocols
- **Message queues** — Kafka, RabbitMQ
- **WebSockets** — long-lived connections
- **CLI tools** — fan out many async operations concurrently

Common thread: **I/O-bound work where you'd otherwise block a thread waiting.**

---

## What is a Future?

A `Future` is a trait:

```rust
trait Future {
    type Output;
    fn poll(&mut self, cx: &mut Context) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),   // done, here's the value
    Pending,    // not done yet, try again later
}
```

A Future is a value you can **poll**. Each poll either returns the finished value or says "not yet."

When you write `async fn`, the compiler rewrites it into a **state machine** that implements `Future`. Every `.await` point becomes a state — the machine remembers where it paused and resumes from there when polled again.

```rust
async fn fetch_and_print(id: i64) -> String {
    let row = db.fetch(id).await;   // state 0: waiting for DB
    let text = process(row).await;  // state 1: waiting for processing
    text                            // state 2: done
}
```

**Futures are lazy** — calling `fetch_and_print(1)` creates the state machine but runs nothing. Only `.await`-ing it (or spawning it) starts execution.

---

## What does `.await` actually do?

`.await` does NOT block the thread. It:

1. Polls the future
2. If `Ready` — takes the value and continues
3. If `Pending` — **suspends the current task** and yields control back to Tokio's scheduler

While your task is suspended, the thread is free to run other tasks. When the I/O completes, Tokio wakes your task and resumes it from where it left off.

From your perspective it looks like "pause here until done." Under the hood, the thread stays busy with other work.

---

## How Tokio works internally

Tokio runs a **thread pool** (default: one thread per CPU core). Each thread runs an event loop:

```
loop {
    poll all ready tasks
    ask the OS: which I/O is done? (epoll on Linux)
    wake tasks whose I/O completed
}
```

The key is `epoll` (Linux) / `kqueue` (macOS) — Tokio registers interest in I/O events with the OS ("tell me when this socket has data"), then parks the thread until the OS signals readiness. No busy-waiting, no polling in a tight loop.

---

## async vs threads — hands-on demo

```rust
use std::time::Duration;

async fn task(id: u32) {
    println!("task {id} starting");
    tokio::time::sleep(Duration::from_secs(1)).await;  // yields, doesn't block
    println!("task {id} done");
}

#[tokio::main]
async fn main() {
    let t1 = tokio::spawn(task(1));
    let t2 = tokio::spawn(task(2));
    let t3 = tokio::spawn(task(3));

    t1.await.unwrap();
    t2.await.unwrap();
    t3.await.unwrap();

    println!("all done");
}
```

All three tasks start immediately and finish after **~1 second total** — not 3. Three tasks ran concurrently on (likely) one thread.

Now swap in `std::thread::sleep`:

```rust
std::thread::sleep(Duration::from_secs(1));  // blocks the thread — no .await
```

Now it takes **~3 seconds**. `std::thread::sleep` blocks the OS thread. While one task sleeps, nothing else can run — you've taken the thread hostage.

**Core rule: never call blocking functions inside async code.** Use the `tokio::` async equivalents:

| Blocking (don't use in async) | Async equivalent |
|---|---|
| `std::thread::sleep` | `tokio::time::sleep` |
| `std::fs::read_to_string` | `tokio::fs::read_to_string` |
| `std::net::TcpListener` | `tokio::net::TcpListener` |

---

## When NOT to use async

Async helps when you're **waiting on I/O** — the thread has nothing to do but wait.

If your work is **CPU-bound** (image resizing, parsing, sorting, crypto), async doesn't help. You're burning CPU cycles, not waiting. Suspending and resuming just adds overhead.

For CPU-bound work:

**`std::thread`** — spawn OS threads, one per core. Share state via `Arc<Mutex<T>>`.

**`rayon`** — data parallelism library. Turns `.iter()` into `.par_iter()` and splits work across cores automatically:

```rust
use rayon::prelude::*;

let sum: u64 = (0..1_000_000u64).into_par_iter().sum();
```

You can mix both — Tokio for I/O, rayon for CPU work. They don't conflict.

### The mental model

```
async / Tokio   →  waiting on I/O    →  one thread, many tasks, cooperative
threads / rayon →  burning CPU       →  many threads, parallel work, preemptive
```

### Escape hatch: `spawn_blocking`

If you must call a blocking function inside async code (e.g., a sync-only library), Tokio provides `spawn_blocking`. It runs the blocking closure on a **separate dedicated thread pool** so it doesn't stall the async scheduler:

```rust
let result = tokio::task::spawn_blocking(|| {
    // blocking work here — runs on a thread, not the async pool
    expensive_cpu_work()
}).await.unwrap();
```

You'll see this in practice with sqlx and some other libraries.

---

## How futures propagate up the call stack

When you `.await` inside an async fn, you're not resolving it yourself — you're saying "poll this, and if it's `Pending`, suspend *me* too." The suspension bubbles up the call stack:

```rust
async fn inner() -> String {
    tokio::time::sleep(Duration::from_secs(1)).await;  // suspends inner
    "done".to_string()
}

async fn outer() -> String {
    inner().await   // suspends outer while inner is pending
}

#[tokio::main]
async fn main() {
    outer().await   // suspends main while outer is pending
}
```

Each `.await` wraps the caller's state machine around the callee's. When `sleep` is `Pending`, `inner` is `Pending`, so `outer` is `Pending`, so `main` is `Pending`.

The chain ends at **Tokio** — which is not async itself. It's regular synchronous code running an event loop that calls `.poll()`:

```
Tokio polls main
  → main polls outer
    → outer polls inner
      → inner polls sleep
        → sleep asks OS to notify when 1s is up → returns Pending
      ← inner returns Pending
    ← outer returns Pending
  ← main returns Pending
← Tokio parks the task, moves on to other work

[1 second later, OS wakes Tokio]

Tokio polls main again → chain resolves bottom-up
```

`#[tokio::main]` is the transition from async back to sync — it calls `runtime.block_on(main())`, a regular blocking call that drives everything above it.

---

## Sequential vs concurrent — the key distinction

A single chain of `.await`s is always **sequential** — top to bottom, in order:

```rust
async fn handler() {
    let a = step_a().await;  // always runs first
    let b = step_b().await;  // always runs second
    let c = step_c().await;  // always runs third
}
```

While `handler` is suspended on `step_a`, it cannot run `step_b` — it has no knowledge of other work.

**Concurrency comes from `tokio::spawn`.** Spawn creates an independent task that Tokio tracks alongside all others. At every `.await` where a task goes `Pending`, Tokio pulls the next ready task from the queue:

```
Tokio queue: [main, t1, t2, t3]

poll t1 → hits sleep → Pending → park t1
poll t2 → hits sleep → Pending → park t2
poll t3 → hits sleep → Pending → park t3
poll main → hits t1.await → Pending → park main

[OS: 1 second elapsed, all sleeps done]

wake t1, t2, t3, main → all go back in queue
poll t1 → Ready ...
```

**The compiler doesn't know which tasks to run while waiting — Tokio does, at runtime**, by maintaining a queue of all spawned tasks and running whichever ones are ready.

For your API: when two users hit `GET /todos` at the same time, axum spawns a handler task for each request. While handler 1 waits on a DB query, Tokio runs handler 2. Neither blocks the other.

---

## `tokio::join!` vs `tokio::spawn`

Both run things concurrently, but differ in ownership and lifetime:

**`tokio::join!`** — runs futures concurrently inside the current task. Waits for all before continuing. Not independent — if one panics, everything stops.

```rust
async fn handler() {
    let (users, todos) = tokio::join!(
        fetch_users(),
        fetch_todos(),
    );
    // both done here, use the results
}
```

**`tokio::spawn`** — creates a fully independent task. Runs detached from the caller. Caller can move on; task lives until it finishes.

```rust
async fn main() {
    let handle = tokio::spawn(background_job());

    do_other_stuff().await;  // runs while background_job is also running

    handle.await.unwrap();   // wait for it later, or never
}
```

| | `join!` | `spawn` |
|---|---|---|
| Scope | Inside current task | Independent task |
| Results | Returns all at once | Via `JoinHandle.await` |
| Lifetime | Tied to current task | Lives independently |
| Overhead | Zero | Small (new task allocation) |
| Use when | You need all results before continuing | Fire-and-forget, or truly independent work |

---

## `spawn` is eager — the exception to laziness

- **`async fn` / `Future`** — lazy. Nothing runs until polled.
- **`tokio::spawn`** — eager. Hands the future to Tokio immediately. Starts running right away, whether or not you ever `.await` the handle.

```rust
let handle = tokio::spawn(background_job());
// background_job is ALREADY running here

do_other_stuff().await;  // runs concurrently with background_job

handle.await.unwrap();   // just collects the result — work was already in progress
```

You can even drop the handle — the task keeps running:

```rust
tokio::spawn(background_job());  // fire and forget
```

Mental model:

```
Future alone     →  lazy, inert, nothing happens
.await on it     →  poll it now, suspend me if Pending
tokio::spawn     →  hand to Tokio, starts immediately, runs independently
```

---

## Concurrent vs parallel

- **Concurrent** — making progress on multiple things by interleaving them, possibly on one thread
- **Parallel** — literally running at the same time on multiple CPU cores

Spawned tasks are concurrent by default. Tokio *may* run them in parallel across threads, but it's not guaranteed. The distinction matters for CPU-bound work (parallel helps) vs I/O-bound work (concurrent is enough).

---

## The async runtime ecosystem

Tokio is the dominant runtime but not the only one:

| Runtime | Notes |
|---|---|
| **Tokio** | De facto standard. Largest ecosystem, most crates target it. Use this. |
| **async-std** | Tried to mirror `std` with async versions. Lost momentum, mostly inactive. |
| **smol** | Minimal, lightweight. Used in constrained environments. |
| **Embassy** | Async for microcontrollers — no OS, no heap. Completely different world. |

The reason multiple runtimes exist: async/await is in the language, but the runtime is not — Rust deliberately left that as a library concern. The `Future` trait is in `std`, so any runtime can poll any future.

In practice the ecosystem has converged on Tokio. Major crates (`axum`, `reqwest`, `sqlx`, `tonic`) all target it. Use anything else and most of the ecosystem won't work.

Some libraries are **runtime-agnostic** — they only use the `Future` trait without depending on Tokio directly. But they're the minority.

---

## Key takeaways

- `async fn` → compiler generates a `Future` (state machine)
- Futures are **lazy** — nothing runs until polled
- `.await` → suspend task if `Pending`, resume when ready (thread stays free)
- Suspended futures bubble up the call stack — Tokio drives the outermost one
- **Tokio** = the runtime that polls futures and manages the thread pool
- **Axum** = web framework built on top of Tokio
- A single chain of `.await`s is always sequential — concurrency requires `spawn`
- `spawn` is eager — task starts immediately, handle is just for collecting results
- `join!` = concurrent within one task; `spawn` = independent task
- Never block a Tokio thread — use async I/O or `spawn_blocking`
- CPU-bound work → threads or rayon, not async
- Tokio is the de facto standard runtime — the ecosystem is built around it
