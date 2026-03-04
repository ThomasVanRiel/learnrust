# Class 12 — Axum in depth, Arc/Mutex, combining Tokio + Axum

## What we did

- Completed Step 2: in-memory todo API with all 5 routes working
- Walked through code annotations — Arc, Mutex, extractors, IntoResponse, derive macros
- Discussed axum's high-level abstraction and how it works at compile time
- Discussed combining raw Tokio TCP with Axum for streaming applications

---

## The in-memory todo API

```rust
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: u64,
    title: String,
    done: bool,
}

type Db = Arc<Mutex<Vec<Todo>>>;

#[tokio::main]
async fn main() {
    let db: Db = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .route("/todos/{id}", get(get_todo).put(update_todo).delete(delete_todo))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
```

Test commands:
```bash
curl -s -X POST http://localhost:3000/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "Buy milk"}' | jq

curl -s http://localhost:3000/todos | jq
curl -s http://localhost:3000/todos/1 | jq

curl -s -X PUT http://localhost:3000/todos/1 \
  -H "Content-Type: application/json" \
  -d '{"done": true}' | jq

curl -X DELETE http://localhost:3000/todos/2
```

---

## Key concepts explained

### `#[derive(...)]` — trait implementations, not methods

`#[derive(Clone, Serialize, Deserialize)]` tells the compiler to generate **trait implementations**, not just methods. `impl Clone for Todo`, `impl Serialize for Todo`, etc. This means `Todo` satisfies those trait contracts everywhere they're expected — not just "has a `.clone()` method".

### `type Db = Arc<Mutex<Vec<Todo>>>` — type alias

A **type alias** — just a name for a long type. No new type is created. Purely cosmetic, saves repeating the full type in every handler signature.

### `Arc<T>` — shared ownership across threads

`Arc` = Atomically Reference Counted. Allows multiple owners of the same heap value across threads. When the last `Arc` is dropped, the value is freed. Like `Rc<T>` but thread-safe.

### `Mutex<T>` — mutual exclusion

Ensures only one task accesses the inner value at a time. `.lock()` blocks until the lock is available, then returns a `MutexGuard`. The guard holds the lock for its entire lifetime — when it's dropped (end of scope), the lock releases automatically. No manual unlock needed.

`.lock()` returns `Err` only if the mutex is **poisoned** — another thread panicked while holding it, leaving data potentially corrupt. Panicking with `.unwrap()` is the right response in that case.

### Extractors — not positional arguments

Axum doesn't care about parameter order in handlers. Each parameter is an **extractor** — axum populates it by type. `State<Db>` can be first or last, it doesn't matter. Axum sees the type, knows how to fill it in.

```rust
// These are equivalent to axum:
async fn handler(State(db): State<Db>, Path(id): Path<u64>) { ... }
async fn handler(Path(id): Path<u64>, State(db): State<Db>) { ... }
```

### Why clone in `list_todos`?

```rust
let todos = db.lock().unwrap();  // MutexGuard — holds the lock
Json(todos.clone())              // clone into owned Vec<Todo>
```

The lock is tied to `todos` (the `MutexGuard`). We need to return data that outlives the lock. `.clone()` creates an independent `Vec<Todo>` that can be returned after the lock drops. Not because `db` is read-only — because owned data must escape the lock's scope.

### `Result<T, E>` as a handler return type

Axum implements `IntoResponse` for `Result<T, E>` where both `T` and `E` implement `IntoResponse`. `StatusCode` and `Json<T>` both do. So axum transparently converts:
- `Ok(Json(todo))` → 200 with JSON body
- `Err(StatusCode::NOT_FOUND)` → 404

Use a tuple `(StatusCode, Json<T>)` when the operation always succeeds. Use `Result<Json<T>, StatusCode>` when it can fail.

### `.find()` → `.cloned()` → `.ok_or()`

```rust
todos.iter()
    .find(|t| t.id == id)   // Option<&Todo>  — reference into the vec
    .cloned()                // Option<Todo>   — owned copy
    .map(Json)               // Option<Json<Todo>>
    .ok_or(StatusCode::NOT_FOUND)  // Result<Json<Todo>, StatusCode>
```

`.find()` returns a reference into the locked vec. `.cloned()` copies it into owned data so it can outlive the lock.

---

## Axum's abstraction level

Axum is deliberately high-level. In most languages, framework magic like extractor injection uses **runtime reflection** — inspecting types while the program runs. Axum does it entirely at **compile time** using the trait system.

Every valid handler signature is proven correct by the compiler:
- Add `Json<Foo>` extractor but `Foo` doesn't implement `Deserialize` → compile error
- Use `State<Db>` but forget `.with_state(db)` → compile error

No runtime surprises. Zero overhead — the abstraction compiles away.

Under the hood, axum sits on top of **hyper** (which does the actual HTTP parsing), which sits on top of Tokio.

| Level | Crate | You handle |
|---|---|---|
| Highest | axum | Business logic only |
| Middle | hyper | HTTP parsing, but no routing |
| Lowest | tokio TCP | Raw bytes, everything |

---

## Combining Axum + raw Tokio TCP

For streaming data (GPS positions, sensor feeds, game state) — raw TCP is the right tool. No HTTP overhead per message, you control the wire format, minimal latency.

You can run both in the same application on the same Tokio runtime:

```rust
#[tokio::main]
async fn main() {
    // REST API on port 3000 — axum handles HTTP
    let api = tokio::spawn(run_api());

    // TCP stream on port 4000 — raw Tokio handles GPS data
    let stream = tokio::spawn(run_tcp_server());

    tokio::join!(api, stream);
}

async fn run_api() {
    let app = Router::new().route("/positions", get(list_positions));
    // ...
}

async fn run_tcp_server() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        tokio::spawn(handle_gps_client(socket, addr));
    }
}
```

Both share the same thread pool and event loop. They can share state via `Arc<Mutex<...>>` — GPS clients write positions over TCP, REST API reads them over HTTP.

Common real-world pattern:
- **Ingest** raw data over a lightweight protocol (TCP, UDP, WebSocket)
- **Serve** processed/queried data over HTTP

---

## Next

Step 3: swap the in-memory `Vec` for SQLite via sqlx — real persistence.
