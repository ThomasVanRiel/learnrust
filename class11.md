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

## Key takeaways

- `async fn` → compiler generates a `Future` (state machine)
- Futures are **lazy** — nothing runs until polled
- `.await` → suspend task if `Pending`, resume when ready (thread stays free)
- **Tokio** = the runtime that polls futures and manages the thread pool
- **Axum** = web framework built on top of Tokio
- Never block a Tokio thread — use async I/O or `spawn_blocking`
- CPU-bound work → threads or rayon, not async
