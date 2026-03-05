# Class 13 — SQLite persistence, polish, and error handling

## What we did

- Replaced in-memory `Vec` with SQLite via sqlx
- Set up connection pool, schema creation, `create_if_missing`
- Implemented all 5 handlers with real SQL queries
- Extracted `find_todo` helper to avoid code duplication
- Added typed request bodies (`CreateTodo`, `UpdateTodo`)
- Replaced `.unwrap()` with proper `ApiError` type and `From` + `IntoResponse`

---

## Final code

```rust
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
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

#[derive(Debug, Deserialize)]
struct CreateTodo {
    title: String,
}

#[derive(Debug, Deserialize)]
struct UpdateTodo {
    title: Option<String>,
    done: Option<bool>,
}

type Db = SqlitePool;

enum ApiError {
    NotFound,
    DatabaseError(sqlx::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND.into_response(),
            ApiError::DatabaseError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> ApiError {
        ApiError::DatabaseError(error)
    }
}

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

async fn list_todos(State(pool): State<Db>) -> Result<Json<Vec<Todo>>, ApiError> {
    let todos = sqlx::query_as!(Todo, "SELECT * FROM todos")
        .fetch_all(&pool)
        .await?;
    Ok(Json(todos))
}

async fn create_todo(
    State(pool): State<Db>,
    Json(payload): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), ApiError> {
    let todo = sqlx::query_as!(
        Todo,
        "INSERT INTO todos (title, done) VALUES (?, false) RETURNING *",
        payload.title
    )
    .fetch_one(&pool)
    .await?;
    Ok((StatusCode::CREATED, Json(todo)))
}

async fn get_todo(State(pool): State<Db>, Path(id): Path<i64>) -> Result<Json<Todo>, ApiError> {
    find_todo(&pool, id)
        .await
        .map(Json)
        .ok_or(ApiError::NotFound)
}

async fn update_todo(
    State(pool): State<Db>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, ApiError> {
    let todo = find_todo(&pool, id).await.ok_or(ApiError::NotFound)?;
    let title = payload.title.unwrap_or(todo.title);
    let done = payload.done.unwrap_or(todo.done);
    let todo = sqlx::query_as!(
        Todo,
        "UPDATE todos SET title = ?, done = ? WHERE id = ? RETURNING *",
        title, done, id,
    )
    .fetch_one(&pool)
    .await?;
    Ok(Json(todo))
}

async fn delete_todo(State(pool): State<Db>, Path(id): Path<i64>) -> Result<StatusCode, ApiError> {
    let result = sqlx::query!("DELETE FROM todos WHERE id = ?", id)
        .execute(&pool)
        .await?;

    if result.rows_affected() > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(ApiError::NotFound)
    }
}
```

---

## Key concepts

### sqlx compile-time SQL checking

`query_as!` and `query!` verify SQL against the real database **at compile time**. Requires `DATABASE_URL` in a `.env` file at the project root:

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

`SqlitePool` is already internally thread-safe and reference-counted. `type Db = SqlitePool` is all you need — no wrapping required.

### `query_as!(Struct, "SQL", args...)` — map rows to structs

| SQLite type | Rust type (sqlx default) |
|---|---|
| `INTEGER` | `i64` |
| `TEXT` | `String` |
| `BOOL` | `bool` |
| Nullable column | `Option<T>` |

Use `NOT NULL` in the schema to get non-optional types. `INTEGER PRIMARY KEY` auto-increments — don't pass `id` on insert.

### `RETURNING *` — get the row back after insert/update

```sql
INSERT INTO todos (title, done) VALUES (?, false) RETURNING *
UPDATE todos SET title = ?, done = ? WHERE id = ? RETURNING *
```

No need for a second `SELECT` query.

### `query!` vs `query_as!`

- `query_as!(Struct, ...)` — maps result rows to a struct
- `query!(...)` — for statements where you don't need rows back (DELETE)

### `rows_affected()` — detect missing rows on DELETE

`DELETE` succeeds even if no rows matched — check `rows_affected()`:

```rust
if result.rows_affected() > 0 {
    Ok(StatusCode::NO_CONTENT)
} else {
    Err(ApiError::NotFound)
}
```

### Typed request bodies

Instead of `serde_json::Value` + manual field extraction, define structs:

```rust
#[derive(Debug, Deserialize)]
struct CreateTodo { title: String }

#[derive(Debug, Deserialize)]
struct UpdateTodo { title: Option<String>, done: Option<bool> }
```

Axum's `Json<CreateTodo>` extractor automatically returns 422 if the body doesn't match the struct. Optional fields use `Option<T>`.

### Custom error type with `From` + `IntoResponse`

The idiomatic pattern for API error handling:

```rust
enum ApiError {
    NotFound,
    DatabaseError(sqlx::Error),
}

// Axum can convert it to a response
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND.into_response(),
            ApiError::DatabaseError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }
}

// ? operator auto-converts sqlx::Error → ApiError
impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> ApiError {
        ApiError::DatabaseError(error)
    }
}
```

With `From` implemented, bare `?` works everywhere — no `map_err` needed:

```rust
let todos = sqlx::query_as!(Todo, "SELECT * FROM todos")
    .fetch_all(&pool)
    .await?;  // sqlx::Error auto-converts to ApiError::DatabaseError
```

### Why `.map(Json)` breaks `?` auto-conversion

```rust
// Doesn't work — map(Json) leaves Err as sqlx::Error, From never fires
.await.map(Json)

// Works — ? triggers From conversion, then wrap in Json
let todo = query.await?;
Ok(Json(todo))
```

The `?` operator triggers `From`. If you transform the value with `.map()` first, `?` never runs and the error type stays as `sqlx::Error`.

### Exposing DB errors to clients

`e.to_string()` in `DatabaseError` response leaks implementation details. In production: log the error internally, return a generic message to the client.

### SQL injection — never use `format!()` for queries

```rust
// WRONG
format!("SELECT * FROM todos WHERE title = '{title}'")

// CORRECT
sqlx::query_as!(Todo, "SELECT * FROM todos WHERE title = ?", title)
```

### `2>&1` in shell

Redirects stderr to stdout. Compiler errors go to stderr — without this, piping to `head` or `grep` shows nothing.

---

## Test commands

```bash
curl -X POST http://localhost:3000/todos -H "Content-Type: application/json" -d '{"title": "Buy milk"}' | jq
curl http://localhost:3000/todos | jq
curl http://localhost:3000/todos/1 | jq
curl -X PUT http://localhost:3000/todos/1 -H "Content-Type: application/json" -d '{"done": true}' | jq
curl -X DELETE http://localhost:3000/todos/1
curl -X DELETE http://localhost:3000/todos/999  # should return 404
```

Avoid `-s` when debugging — it silences errors. Use `-v` for verbose output.

---

## Next

Project 3 complete. Next: recap sessions on traits, lifetimes, and other concepts before Project 4.
