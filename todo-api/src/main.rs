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

// Data model
// ! Let the compile generate debug, clone, (de)serialize methods.
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

// Shared state
type Db = SqlitePool;

enum ApiError {
    NotFound,
    DatabaseError(sqlx::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND).into_response(),
            ApiError::DatabaseError(body) => {
                (StatusCode::INTERNAL_SERVER_ERROR, body.to_string()).into_response()
            }
        }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> ApiError {
        ApiError::DatabaseError(error)
    }
}

// Main function
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // ? Should I update the main() signature to return result?
    let opts = SqliteConnectOptions::from_str("sqlite://todo.db")?.create_if_missing(true);

    let pool: Db = SqlitePool::connect_with(opts).await?;
    // Just check it the table exists or create one, we don't care about the result
    let _ = sqlx::query(
        "CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY, title TEXT NOT NULL, done BOOL NOT NULL)",
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
async fn list_todos(State(pool): State<Db>) -> Result<Json<Vec<Todo>>, ApiError> {
    let todos: Vec<Todo> = sqlx::query_as!(Todo, "SELECT * FROM todos")
        .fetch_all(&pool)
        .await?;
    Ok(Json(todos))
}

async fn create_todo(
    State(pool): State<Db>,
    Json(payload): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), ApiError> {
    let title = payload.title;
    let todo: Todo = sqlx::query_as!(
        Todo,
        "INSERT INTO todos (title, done) VALUES (?, false) RETURNING *",
        title
    )
    .fetch_one(&pool)
    .await?;
    Ok((StatusCode::CREATED, Json(todo)))
}

async fn find_todo(pool: &Db, id: i64) -> Option<Todo> {
    sqlx::query_as!(Todo, "SELECT * from todos WHERE id = ?", id)
        .fetch_one(pool)
        .await
        .ok()
}
// ? Why does this function return Result<> while create returns tuple? Is it because GET must
// return something while POST does not? Is the Err unwrapped by axum?
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
        title,
        done,
        id,
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
