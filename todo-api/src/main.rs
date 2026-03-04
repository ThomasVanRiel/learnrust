use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

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
type Db = Arc<Mutex<Vec<Todo>>>;

// Main function
#[tokio::main]
async fn main() {
    // ! Create shared memory accessible by all threads containing a Vec which can only be accessed
    // by one task at the time.
    // ? What does Mutex raise when multiple tasks try to access it?
    let db: Db = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        // ! Bind get and post calls to /todos
        .route("/todos", get(list_todos).post(create_todo))
        // ! Bind get, put, and delete calls /todos/{id}
        .route(
            "/todos/{id}",
            get(get_todo).put(update_todo).delete(delete_todo),
        )
        // ! Passes State(db) as the first argument in the REST calls
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

// Handlers
async fn list_todos(State(db): State<Db>) -> Json<Vec<Todo>> {
    // ? Lock Mutex for lifetime of todos
    // ? Why is unwrapping and panicking ok here?
    let todos = db.lock().unwrap();
    // ! Return copy of todos because db is read only
    // ! todos can be serialized to Json because the compile derived the methods
    Json(todos.clone())
}

async fn create_todo(
    State(db): State<Db>,
    Json(payload): Json<serde_json::Value>,
) -> (StatusCode, Json<Todo>) {
    let mut todos = db.lock().unwrap();
    // ! Cast usize to u64 and increment to new index
    let id = todos.len() as u64 + 1;
    let todo = Todo {
        id,
        title: payload["title"].as_str().unwrap_or("untitled").to_string(),
        done: false,
    };
    todos.push(todo.clone());
    (StatusCode::CREATED, Json(todo))
}

// ? Why does this function return Result<> while create returns tuple? Is it because GET must
// return something while POST does not? Is the Err unwrapped by axum?
async fn get_todo(State(db): State<Db>, Path(id): Path<u64>) -> Result<Json<Todo>, StatusCode> {
    let todos = db.lock().unwrap();
    todos
        .iter()
        .find(|t| t.id == id)
        // ! Clone because it needs to be owned to map
        .cloned()
        .map(Json)
        // ! If Err, return 404
        .ok_or(StatusCode::NOT_FOUND)
}

async fn update_todo(
    State(db): State<Db>,
    Path(id): Path<u64>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<Todo>, StatusCode> {
    let mut todos = db.lock().unwrap();
    // ! If id not found, return Err(404)
    let todo = todos
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or(StatusCode::NOT_FOUND)?;
    // ! Check if title is in payload and update if present
    if let Some(title) = payload["title"].as_str() {
        todo.title = title.to_string();
    }
    // ! Check if done is in payload and update if present
    if let Some(done) = payload["done"].as_bool() {
        todo.done = done;
    }
    // ! Return owned todo object
    Ok(Json(todo.clone()))
}

async fn delete_todo(State(db): State<Db>, Path(id): Path<u64>) -> StatusCode {
    let mut todos = db.lock().unwrap();
    let before = todos.len();
    // Keep every todo with id mismatch
    todos.retain(|t| t.id != id);
    // If length is the same, the id was not deleted and hence not present in todos
    if todos.len() < before {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
