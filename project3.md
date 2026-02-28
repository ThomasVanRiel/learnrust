# Project 3 — Todo REST API

A JSON REST API backed by SQLite. Introduces async Rust, tokio, axum, and sqlx.

---

## Stack

| Crate | Role |
|---|---|
| `tokio` | Async runtime — drives the event loop |
| `axum` | Web framework — routing, handlers, extractors |
| `serde` / `serde_json` | JSON serialization (already familiar from csvtool) |
| `sqlx` | Async database access with compile-time checked SQL |
| SQLite | Database — file-based, no server needed |

---

## API Design

```
GET    /todos        → list all todos
POST   /todos        → create a todo       (JSON body)
GET    /todos/:id    → get one todo
PUT    /todos/:id    → update a todo       (JSON body)
DELETE /todos/:id    → delete a todo
```

Request/response format: JSON throughout.

---

## Async Rust — the core concept

### Why async?

A web server handles many concurrent requests. One OS thread per request doesn't scale — threads are expensive (~8MB stack each). Async lets you handle thousands of concurrent connections on a small thread pool by suspending work that's blocked on I/O and doing other work in the meantime.

### Futures

An `async fn` returns a **Future** — a value representing work not yet complete:

```rust
async fn fetch_todo(id: i64) -> Todo {
    let todo = db.fetch_one(id).await; // suspended here while DB query runs
    todo
}
```

**Futures are lazy** — calling `fetch_todo(1)` does nothing. It just creates a Future. Only `.await`-ing it actually executes it.

`.await` suspends the current task until the future resolves. While suspended, the thread is free to run other tasks. This is cooperative multitasking.

### The runtime

Rust has async/await syntax built in, but no built-in runtime. **Tokio** provides the runtime — the scheduler that drives futures to completion and manages the thread pool.

```rust
#[tokio::main]           // macro that sets up the tokio runtime
async fn main() {
    // can use .await here
}
```

Without `#[tokio::main]`, you can't `.await` anything in `main`.

### async vs threads — mental model

| | Threads | Async |
|---|---|---|
| Scheduling | OS preemptive | Cooperative (yield at `.await`) |
| Cost per task | ~8MB stack | ~KB per future |
| Good for | CPU-bound work | I/O-bound work (network, disk, DB) |
| Rust primitive | `std::thread` | `async`/`await` + tokio |

For a web API (mostly I/O-bound: waiting on DB queries, network), async is the right choice.

---

## Project structure (target)

```
todo-api/
├── Cargo.toml
├── todo.db             ← SQLite database file (created at runtime)
└── src/
    └── main.rs         ← start here, split into modules later
```

---

## Cargo.toml dependencies

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
axum = "0.8"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.8", features = ["sqlite", "runtime-tokio", "macros"] }
```

---

## Build plan

1. **Hello World** — minimal axum server, one route, confirm it runs
2. **In-memory todos** — routes + handlers with a `Vec` (no DB yet), learn axum patterns
3. **Add SQLite** — swap in sqlx, create the schema, wire up real persistence
4. **Polish** — proper error handling, 404s, validation

---

## Axum concepts

### Handler functions

A handler is an `async fn` that returns something axum can turn into a response:

```rust
async fn hello() -> &'static str {
    "Hello, world!"
}
```

### Extractors

Axum uses **extractors** to pull data out of requests:

```rust
async fn get_todo(
    Path(id): Path<i64>,          // extracts :id from the URL
    State(db): State<DbPool>,     // extracts shared app state
) -> Json<Todo> {
    // ...
}
```

Extractors are just function parameters — axum figures out how to populate them. If extraction fails (bad JSON, wrong type), axum returns an error automatically.

### State

Shared state (like a DB connection pool) is passed to handlers via `State`:

```rust
let app = Router::new()
    .route("/todos", get(list_todos))
    .with_state(pool);  // pool available to all handlers via State(pool)
```

### Routing

```rust
let app = Router::new()
    .route("/todos",     get(list_todos).post(create_todo))
    .route("/todos/:id", get(get_todo).put(update_todo).delete(delete_todo));
```

---

## sqlx concepts

### Compile-time checked SQL

sqlx verifies your SQL queries against the actual database schema **at compile time**. Typos in column names, wrong types — caught before the program runs.

Requires a database to exist at compile time (or a saved query cache — `sqlx prepare`).

### Query macros

```rust
// returns a Vec<Todo>
let todos = sqlx::query_as!(Todo, "SELECT * FROM todos")
    .fetch_all(&pool)
    .await?;

// insert and return the new row
let todo = sqlx::query_as!(
    Todo,
    "INSERT INTO todos (title, done) VALUES (?, ?) RETURNING *",
    title, false
)
.fetch_one(&pool)
.await?;
```

`query_as!` maps rows directly to a struct — same idea as serde's `Deserialize`, but for SQL rows.

---

## Key concepts to learn in this project

- `async fn`, `await`, `Future`
- `#[tokio::main]`
- axum: `Router`, `route()`, handler functions, extractors (`Path`, `Json`, `State`)
- `Json<T>` — deserializing request bodies, serializing responses
- `sqlx::query_as!` macro, connection pools (`SqlitePool`)
- Error handling in async context — `?` still works, but return types need care
- `Arc<T>` — sharing state across async tasks (brief intro)
