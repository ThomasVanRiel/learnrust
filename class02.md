# Class 02 — Error Handling, Pattern Matching & Structs

**Date:** 2026-02-18

## Topics Covered

### 1. Homework Review: Ownership & Vec Indexing

Removed `&` from `let query = &args[1]` and ran `cargo check`:

```
error[E0507]: cannot move out of index of `Vec<String>`
```

**Why?** Indexing a Vec with `args[1]` tries to *move* the value out of the Vec. This would leave a "hole" — an invalid slot. Rust prevents this.

**Ways to take ownership from a Vec:**

| Method | What it does |
|---|---|
| `.remove(i)` | Removes element at index, shifts rest down |
| `.pop()` | Takes the last element off |
| `.into_iter()` | Consumes the entire Vec, yields owned elements |

**Takeaway:** Ask yourself "do I need ownership, or just a reference?" Most of the time, borrowing with `&` is enough.

### 2. Neovim/LazyVim Setup for Rust

Running cargo commands from Neovim:

| Command | Behavior |
|---|---|
| `:!cargo check` | Runs inline, output at bottom (may be swallowed by noice.nvim in LazyVim) |
| `:term cargo check` | Opens a terminal buffer |
| `Ctrl-/` | LazyVim floating terminal — best option for LazyVim users |
| `:set makeprg=cargo\ check` then `:make` | Loads errors into quickfix list (`:copen`, `:cn`/`:cp`) |

**Note:** LazyVim's noice.nvim plugin overrides the command-line UI and can hide `:!` output. Use the floating terminal (`Ctrl-/`) instead.

**Working directory:** If Neovim opens at `learnrust/` but cargo needs to run from `rgrep/`, either:
- Prefix commands: `:!cd rgrep && cargo check`
- Change Neovim's directory: `:cd rgrep` (verify with `:pwd`)

### 3. `Result<T, E>` — Rust's Error Handling

Rust has **no exceptions**. Errors are values you must handle explicitly.

```rust
enum Result<T, E> {
    Ok(T),    // success — holds the value
    Err(E),   // failure — holds the error
}
```

`fs::read_to_string()` returns `Result<String, io::Error>`:

```rust
// Before — crashes on error:
let contents = fs::read_to_string(filename)
    .expect("Should have been able to read the file");

// After — handles error gracefully:
let contents = match fs::read_to_string(filename) {
    Ok(text) => text,
    Err(error) => {
        println!("Error reading file '{}': {}", filename, error);
        return;
    }
};
```

**Compiler trick:** To discover a function's return type, assign it to the wrong type:

```rust
let contents: i32 = fs::read_to_string(filename);
// Compiler error tells you: expected `Result<String, io::Error>`, found `i32`
```

### 4. `Option<T>` — Rust's Replacement for Null

There is no `null` in Rust. If a value might not exist, the type system forces you to handle it:

```rust
enum Option<T> {
    Some(T),  // there's a value
    None,     // there's nothing
}
```

Vec's `.get()` method returns `Option<&T>` instead of panicking on out-of-bounds:

```rust
// Before — panics if args[1] doesn't exist:
let query = &args[1];

// After — handles missing args:
let query = match args.get(1) {
    Some(q) => q,
    None => {
        println!("Usage: rgrep <query> <filename>");
        return;
    }
};
```

### 5. `match` — Pattern Matching

`match` is like `switch` on steroids. It must be **exhaustive** — every possible case must be handled.

**Matching on two values at once with a tuple:**

```rust
let (query, filename) = match (args.get(1), args.get(2)) {
    (Some(q), Some(f)) => (q, f),
    _ => {
        println!("Usage: rgrep <query> <filename>");
        return;
    }
};
```

Key concepts:
- **Destructuring** — pulling values out of tuples, enums, and structs inside a `match`.
- **`_` wildcard** — catch-all pattern, matches anything. Like `default:` in a switch.
- **Exhaustiveness** — the compiler ensures every variant is handled.

### 6. Structs

Defined a `Config` struct to group related data:

```rust
struct Config {
    query: String,
    filename: String,
}
```

**Why owned `String` instead of `&str`?** The struct should own its data. If it only borrowed, it couldn't outlive the source (would need lifetime annotations — a later topic).

**Creating an instance:**

```rust
Config {
    query: q.to_string(),
    filename: f.to_string(),
}
```

`.to_string()` creates an owned `String` from a `&str` or `&String`. `.clone()` also works.

### 7. Functions Returning `Result`

Wrote a `parse_config` function that returns `Result<Config, String>`:

```rust
fn parse_config(args: &Vec<String>) -> Result<Config, String> {
    match (args.get(1), args.get(2)) {
        (Some(q), Some(f)) => Ok(Config {
            query: q.to_string(),
            filename: f.to_string(),
        }),
        _ => Err(String::from("Usage: rgrep <query> <filename>"))
    }
}
```

Called from `main()`:

```rust
let config = match parse_config(&args) {
    Ok(config) => config,
    Err(error) => {
        println!("{}", error);
        return;
    }
};
```

Then use `config.query` and `config.filename` in the rest of the code.

### 8. Implicit Returns (Expression vs Statement)

In Rust, the last expression **without a semicolon** is the return value:

```rust
// This WORKS — match is the tail expression, its value is returned:
fn parse_config(args: &Vec<String>) -> Result<Config, String> {
    match (args.get(1), args.get(2)) {
        (Some(q), Some(f)) => Ok(Config { ... }),
        _ => Err(String::from("..."))
    }
}

// This FAILS — semicolon makes it a statement, function returns ():
fn parse_config(args: &Vec<String>) -> Result<Config, String> {
    match (args.get(1), args.get(2)) {
        (Some(q), Some(f)) => Ok(Config { ... }),
        _ => Err(String::from("..."))
    };  // <-- this semicolon breaks it
}
```

**Rule:** Semicolon = statement (throws away the value). No semicolon = expression (value is used).

This is why `return` is rarely used in Rust — the idiomatic way is to let the last expression be the return value.

## Current State of Code

```rust
use std::env;
use std::fs;

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &Vec<String>) -> Result<Config, String> {
    match (args.get(1), args.get(2)) {
        (Some(q), Some(f)) => Ok(Config {
            query: q.to_string(),
            filename: f.to_string(),
        }),
        _ => Err(String::from("Usage: rgrep <query> <filename>"))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match parse_config(&args) {
        Ok(config) => config,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };

    let contents = match fs::read_to_string(&config.filename) {
        Ok(text) => text,
        Err(error) => {
            println!("Error reading file '{}': {}", &config.filename, error);
            return;
        }
    };

    for line in contents.lines() {
        if line.contains(&config.query) {
            println!("{line}");
        }
    }
}
```

## Where We Left Off

- Completed Steps 1-3 (hardcoded search, CLI args + file reading, error handling).
- Started Step 4: created `Config` struct and `parse_config` function.
- **Currently working on:** Moving `parse_config` into an `impl Config` block (associated functions).

## Next Steps

- `impl` blocks — attach `parse_config` to `Config` as an associated function.
- The `?` operator — shorthand for matching on `Result`/`Option`.
- Better error types — beyond `String` as the error type.
