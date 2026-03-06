# Class 18 — Error handling patterns

## Plan

- Recap: `Result`, `?`, `From`
- When `.unwrap()` is acceptable
- Custom error types by hand (recap from todo API)
- `thiserror` — derive macros for error types
- `anyhow` — easy error handling for applications
- When to use which

---

## Recap: `Result` and `?`

```rust
fn read_file(path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(path)?;  // ? returns early on Err
    Ok(content)
}
```

`?` does two things:
1. If `Ok(v)` — unwraps to `v`, continues
2. If `Err(e)` — calls `From::from(e)` to convert the error, then returns early

---

## When `.unwrap()` is acceptable

- In tests — panicking is fine
- In `main` during startup — if the program can't start, crashing is right
- When you've already checked the value (`if let Some(x)` then `.unwrap()`)
- Prototyping — replace with proper handling before production

Never in request handlers, library code, or anywhere errors should be recoverable.

---

## Custom error types by hand

You did this in the todo API:

```rust
enum ApiError {
    NotFound,
    DatabaseError(sqlx::Error),
}

impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        ApiError::DatabaseError(e)
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ApiError::NotFound => write!(f, "not found"),
            ApiError::DatabaseError(e) => write!(f, "database error: {e}"),
        }
    }
}
```

Works fine but verbose. `thiserror` automates this.

---

## `thiserror` — derive macros for error types

Add to `Cargo.toml`:
```toml
thiserror = "1"
```

The same error type with `thiserror`:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum ApiError {
    #[error("not found")]
    NotFound,

    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
```

`#[error("...")]` generates `Display`. `#[from]` generates `From`. That's it.

The `{0}` refers to the first field of the variant — same as format string interpolation.

### `thiserror` is for libraries and typed errors

Use it when:
- You're writing a library and callers need to handle specific error cases
- You want callers to `match` on error variants
- You need precise, structured errors

---

## `anyhow` — easy error handling for applications

Add to `Cargo.toml`:
```toml
anyhow = "1"
```

```rust
use anyhow::{Result, Context};

fn read_config(path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read config from {path}"))?;
    let config: Config = serde_json::from_str(&content)
        .context("failed to parse config")?;
    Ok(config)
}
```

`anyhow::Result<T>` is `Result<T, anyhow::Error>`. `anyhow::Error` can hold **any** error type — no `From` implementations needed. `?` just works with any error.

`.context("...")` adds a human-readable message to the error chain.

### `anyhow` is for applications

Use it when:
- You're writing a binary (CLI tool, server) not a library
- You don't need callers to match on specific error variants
- You want to quickly propagate and annotate errors without boilerplate
- You want an error chain for debugging (`anyhow` preserves the full chain)

---

## Comparing error chains

```
Error: failed to read config from config.toml

Caused by:
    No such file or directory (os error 2)
```

`anyhow` gives you this automatically. With manual error types you'd have to build it yourself.

---

## When to use which

| Scenario | Use |
|---|---|
| Quick prototyping | `.unwrap()` / `expect()` |
| Application (binary) | `anyhow` |
| Library with typed errors | `thiserror` |
| Library + application | `thiserror` internally, `anyhow` in `main` |
| API with typed HTTP errors | Custom type + `thiserror` + `IntoResponse` |

The todo API used a custom type because it needed `IntoResponse` — axum needs to know how to turn errors into HTTP responses. `anyhow::Error` doesn't implement `IntoResponse`. In real projects you'd combine: `thiserror` for the error enum, `IntoResponse` for the axum conversion.

---

## Additional notes from session

### `anyhow` — no boilerplate, rich error chains

`anyhow::Result<T>` is `Result<T, anyhow::Error>`. Any error type works with `?` — no `From` impls needed.

`.context("...")` adds a human-readable layer:

```rust
fn read_number(path: &str) -> anyhow::Result<i32> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read {path}"))?;
    let number = content.trim().parse::<i32>()
        .context("failed to parse number")?;
    Ok(number)
}
```

Output on failure:
```
Error: failed to read config.toml

Caused by:
    No such file or directory (os error 2)
```

### `thiserror` — structured errors without boilerplate

`#[from]` generates `From` — `?` auto-converts. `#[error("...")]` generates `Display`. No manual `impl` blocks needed:

```rust
#[derive(Error, Debug)]
enum AppError {
    #[error("file not found: {0}")]
    NotFound(String),

    #[error("parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
```

With `#[from]`, `?` converts `io::Error` → `AppError::Io` and `ParseIntError` → `AppError::Parse` automatically. No `.map_err()` or `.context()` needed.

Tradeoff: you lose human-readable context messages — you get the raw error from the underlying type.

### The core tension

- `anyhow` — less code, rich `.context()` messages, but callers can't `match` on variants
- `thiserror` — structured variants callers can match on, explicit error type required
- In real projects: `thiserror` for the error enum, `anyhow` in `main` / application code

---

## Exercises completed

- `read_number` with `anyhow` — `.context()` / `.with_context()`, `?` across different error types
- Rewrote with `thiserror` — `#[from]` variants, `?` with no `map_err`, clean two-liner function body

---

## Next

Project 4: CHIP-8 Emulator — fetch/decode/execute loop, bit manipulation, concurrency (timers), `Box<dyn Renderer>` trait object, minifb renderer, then SDL2 via FFI.
