use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;

// Data model
// ! Let the compile generate debug, clone, (de)serialize methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: u64,
    title: String,
    done: bool,
}

// Shared state
// ! Define a new type for shorthand notation later
type Db = SqlitePool;

// Main function
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // ? Should I update the main() signature to return result?
    let opts = SqliteConnectOptions::from_str("sqlite://todo.db")?.create_if_missing(true);

    let pool: Db = SqlitePool::connect_with(opts).await?;
    // Just check it the table exists or create one, we don't care about the result
    let _ = sqlx::query(
        "CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY, title TEXT, done BOOL)",
    )
    .execute(&pool)
    .await;

    let app = Router::new()
        // ! Bind get and post calls to /todos
        .route("/todos", get(list_todos).post(create_todo))
        // ! Bind get, put, and delete calls /todos/{id}
        .route(
            "/todos/{id}",
            get(get_todo).put(update_todo).delete(delete_todo),
        )
        // ! Passes State(db) as the first argument in the REST calls
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

// Handlers
async fn list_todos(State(db): State<Db>) -> Json<Vec<Todo>> {
    todo!();
}

async fn create_todo(
    State(db): State<Db>,
    Json(payload): Json<serde_json::Value>,
) -> (StatusCode, Json<Todo>) {
    todo!();
}

// ? Why does this function return Result<> while create returns tuple? Is it because GET must
// return something while POST does not? Is the Err unwrapped by axum?
async fn get_todo(State(db): State<Db>, Path(id): Path<u64>) -> Result<Json<Todo>, StatusCode> {
    todo!();
}

async fn update_todo(
    State(db): State<Db>,
    Path(id): Path<u64>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<Todo>, StatusCode> {
    todo!();
}

async fn delete_todo(State(db): State<Db>, Path(id): Path<u64>) -> StatusCode {
    todo!();
}
