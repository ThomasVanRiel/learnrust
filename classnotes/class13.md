# Class 13 — SQLite persistence with sqlx

## What we did

- Replaced in-memory `Vec` with SQLite via sqlx
- Set up connection pool, schema creation, `create_if_missing`
- Implemented all 5 handlers with real SQL queries
- Learned sqlx compile-time checking, `query_as!`, `query!`, `RETURNING`
- Extracted `find_todo` helper to avoid code duplication

---

## Final code

```rust
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: i64,
    title: String,
    done: bool,
}

type Db = SqlitePool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let opts = SqliteConnectOptions::from_str("sqlite://todo.db")?.create_if_missing(true);
    let pool: Db = SqlitePool::connect_with(opts).await?;

    let _ = sqlx::query(
        "CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY, title TEXT NOT NULL, done BOOL NOT NULL)",
    )
    .execute(&pool)
    .await;

    let app = Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .route("/todos/{id}", get(get_todo).put(update_todo).delete(delete_todo))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn find_todo(pool: &Db, id: i64) -> Option<Todo> {
    sqlx::query_as!(Todo, "SELECT * FROM todos WHERE id = ?", id)
        .fetch_one(pool)
        .await
        .ok()
}

async fn list_todos(State(pool): State<Db>) -> Json<Vec<Todo>> {
    let todos: Vec<Todo> = sqlx::query_as!(Todo, "SELECT * FROM todos")
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(todos)
}

async fn create_todo(
    State(pool): State<Db>,
    Json(payload): Json<serde_json::Value>,
) -> (StatusCode, Json<Todo>) {
    let title = payload["title"].as_str().unwrap_or("untitled").to_string();
    let todo: Todo = sqlx::query_as!(
        Todo,
        "INSERT INTO todos (title, done) VALUES (?, false) RETURNING *",
        title
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    (StatusCode::CREATED, Json(todo))
}

async fn get_todo(State(pool): State<Db>, Path(id): Path<i64>) -> Result<Json<Todo>, StatusCode> {
    find_todo(&pool, id)
        .await
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn update_todo(
    State(pool): State<Db>,
    Path(id): Path<i64>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<Todo>, StatusCode> {
    let todo = find_todo(&pool, id).await.ok_or(StatusCode::NOT_FOUND)?;
    let title = payload["title"].as_str().unwrap_or(&todo.title);
    let done = payload["done"].as_bool().unwrap_or(todo.done);
    sqlx::query_as!(
        Todo,
        "UPDATE todos SET title = ?, done = ? WHERE id = ? RETURNING *",
        title,
        done,
        id,
    )
    .fetch_one(&pool)
    .await
    .map(Json)
    .map_err(|_| StatusCode::NOT_FOUND)
}

async fn delete_todo(State(pool): State<Db>, Path(id): Path<i64>) -> StatusCode {
    match sqlx::query!("DELETE FROM todos WHERE id = ?", id)
        .execute(&pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                StatusCode::NO_CONTENT
            } else {
                StatusCode::NOT_FOUND
            }
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
```

---

## Key concepts

### sqlx compile-time SQL checking

`query_as!` and `query!` are macros that verify SQL against the real database **at compile time**. Typos in column names, wrong types — caught before the program runs.

Requires `DATABASE_URL` set in a `.env` file at the project root:
```
DATABASE_URL=sqlite://todo.db
```

The database file must exist when compiling. Run the server once first to create it.

### `SqliteConnectOptions` — create if missing

```rust
let opts = SqliteConnectOptions::from_str("sqlite://todo.db")?.create_if_missing(true);
let pool = SqlitePool::connect_with(opts).await?;
```

Without `create_if_missing(true)`, sqlx fails if the file doesn't exist.

### `SqlitePool` replaces `Arc<Mutex<Vec<Todo>>>`

`SqlitePool` is already internally thread-safe and reference-counted — no need to wrap it in `Arc<Mutex<...>>`. Just clone and pass it around freely. `type Db = SqlitePool` is all you need.

### `query_as!(Struct, "SQL", args...)` — map rows to structs

Maps SQL rows directly to a Rust struct. Field names must match column names exactly. Types must be compatible:

| SQLite type | Rust type (sqlx default) |
|---|---|
| `INTEGER` | `i64` |
| `TEXT` | `String` |
| `BOOL` | `bool` |
| Nullable column | `Option<T>` |

Use `NOT NULL` in the schema to get non-optional types. `INTEGER PRIMARY KEY` auto-increments — don't pass `id` on insert.

### `RETURNING *` — get the inserted/updated row back

```sql
INSERT INTO todos (title, done) VALUES (?, false) RETURNING *
UPDATE todos SET title = ?, done = ? WHERE id = ? RETURNING *
```

Returns the full row after the operation. No need for a second `SELECT` query.

### `query!` vs `query_as!`

- `query_as!(Struct, ...)` — maps result rows to a struct
- `query!(...)` — for queries where you don't need rows back (DELETE, or just checking `rows_affected()`)

### `rows_affected()` — detect missing rows on DELETE

`DELETE` succeeds even if no rows matched. Check `rows_affected()` to distinguish "deleted" from "not found":

```rust
match sqlx::query!("DELETE FROM todos WHERE id = ?", id)
    .execute(&pool)
    .await
{
    Ok(result) => {
        if result.rows_affected() > 0 { StatusCode::NO_CONTENT }
        else { StatusCode::NOT_FOUND }
    }
    Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
}
```

### `find_todo` helper — avoid duplication

Both `get_todo` and `update_todo` need to look up a todo by id. Extracting it avoids repeating the query:

```rust
async fn find_todo(pool: &Db, id: i64) -> Option<Todo> {
    sqlx::query_as!(Todo, "SELECT * FROM todos WHERE id = ?", id)
        .fetch_one(pool)
        .await
        .ok()  // converts Result → Option, discarding the error
}
```

`.ok()` on `Result<T, E>` converts it to `Option<T>` — `Ok(v)` → `Some(v)`, `Err(_)` → `None`.

### Panic in a handler doesn't kill the server

Each request runs in its own `tokio::spawn`ed task. A panic kills that task (returns 500) but the runtime and other tasks keep running. The server stays up.

### SQL injection — never use `format!()` for queries

Always use `?` placeholders:
```rust
// WRONG — SQL injection risk
format!("SELECT * FROM todos WHERE title = '{title}'")

// CORRECT — sqlx escapes the value
sqlx::query_as!(Todo, "SELECT * FROM todos WHERE title = ?", title)
```

### `2>&1` in shell

Redirects stderr to stdout so both are captured together. Compiler errors go to stderr — without this, piping to `head` or `grep` might show nothing.

---

## Test commands

```bash
# Create
curl -X POST http://localhost:3000/todos -H "Content-Type: application/json" -d '{"title": "Buy milk"}' | jq

# List
curl http://localhost:3000/todos | jq

# Get one
curl http://localhost:3000/todos/1 | jq

# Update
curl -X PUT http://localhost:3000/todos/1 -H "Content-Type: application/json" -d '{"done": true}' | jq

# Delete
curl -X DELETE http://localhost:3000/todos/1

# Delete missing id — should return 404
curl -X DELETE http://localhost:3000/todos/999
```

Note: avoid `-s` when debugging — it silences errors. Use `-v` for full verbose output.

---

## Next

Step 4: polish — proper error handling (no more `.unwrap()` in handlers), typed request bodies instead of `serde_json::Value`, real 400 responses for bad input.
