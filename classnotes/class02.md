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

### 9. `impl` Blocks — Associated Functions & Methods

Rust separates data (`struct`) from behavior (`impl`):

```rust
impl Config {
    // Associated function (no self) — like a static method
    // Called with: Config::new(&args)
    fn new(args: &Vec<String>) -> Result<Config, String> { ... }

    // Method (takes &self) — called on an instance
    // Called with: config.search(&contents)
    fn search(&self, contents: &str) -> Vec<String> { ... }
}
```

| | Associated function | Method |
|---|---|---|
| First parameter | no `self` | `&self`, `&mut self`, or `self` |
| Call syntax | `Type::func()` | `instance.method()` |
| C++ analogy | `static` method | instance method |

`&self` is shorthand for `self: &Config` — an immutable borrow of the instance.

**Key difference from C++/Python:** Rust has no classes. `struct` defines data, `impl` adds behavior. No inheritance — polymorphism uses traits instead.

### 10. The `?` Operator

Shorthand for "unwrap `Ok` or return `Err` early":

```rust
// Without ? — verbose match
let contents = match fs::read_to_string(&config.filename) {
    Ok(text) => text,
    Err(error) => return Err(error),
};

// With ? — one line
let contents = fs::read_to_string(&config.filename)?;
```

**Requirement:** The function using `?` must return `Result`. That's why we extracted `run()`:

```rust
fn run(config: &Config) -> Result<(), String> {
    let contents = fs::read_to_string(&config.filename)
        .map_err(|e| e.to_string())?;
    // ...
    Ok(())
}
```

- `()` is the **unit type** — like `void` in C++. `Result<(), String>` means "success with no value, or an error string."
- `.map_err(|e| e.to_string())` converts `io::Error` to `String` to match the return type.
- `|e| e.to_string()` is a **closure** (anonymous function) — `|params| body`.

### 11. `if let` — Partial Pattern Matching

When you only care about one variant of a match:

```rust
// Full match — verbose for one case
match run(&config) {
    Ok(()) => (),
    Err(error) => { eprintln!("{}", error); }
};

// if let — cleaner
if let Err(error) = run(&config) {
    eprintln!("{}", error);
}
```

### 12. Custom Enums

Defined `SearchMode` to support `-i` flag:

```rust
enum SearchMode {
    CaseSensitive,
    CaseInsensitive,
}
```

Used in `search()` to branch behavior:

```rust
let is_match = match &self.mode {
    SearchMode::CaseSensitive => line.contains(self.query.as_str()),
    SearchMode::CaseInsensitive => line.to_lowercase().contains(&self.query.to_lowercase()),
};
```

### 13. Iterator Chains

Parsed CLI flags using iterator methods:

```rust
let has_i_flag = args.iter().any(|a| a == "-i");

let non_flags: Vec<&String> = args.iter()
    .skip(1)                          // skip program name
    .filter(|a| !a.starts_with("-")) // keep non-flag args
    .collect();                       // build into Vec
```

| Method | Purpose |
|---|---|
| `.iter()` | Create an iterator over references |
| `.any(\|a\| ...)` | True if any element matches |
| `.skip(n)` | Skip first n elements |
| `.filter(\|a\| ...)` | Keep elements matching predicate |
| `.collect()` | Build a collection from iterator |
| `.enumerate()` | Yield `(index, value)` pairs |

All lazy — nothing runs until `.collect()` or another consumer pulls values through.

### 14. Line Numbers with `.enumerate()`

```rust
for (index, line) in contents.lines().enumerate() {
    // index is 0-based
    let prefix = if self.line_numbers {
        format!("{}:", index + 1)  // 1-based for display
    } else {
        String::new()
    };
    matches.push(format!("{prefix}{line}"));
}
```

`format!()` is like `println!()` but returns a `String` instead of printing.

### 15. Testing

Rust has built-in testing — no external framework needed:

```rust
#[cfg(test)]           // only compiled during `cargo test`
mod tests {
    use super::*;      // import everything from parent module

    #[test]
    fn case_sensitive_search() {
        let config = Config {
            query: String::from("hello"),
            filename: String::from("test.txt"),
            mode: SearchMode::CaseSensitive,
            line_numbers: false,
        };

        let contents = "hello world\nGoodbye world\nhello again";
        let results = config.search(contents);

        assert_eq!(results, vec!["hello world", "hello again"]);
    }
}
```

- `#[test]` — marks a function as a test
- `assert_eq!(a, b)` — panics (fails) if `a != b`
- `vec!["a", "b"]` — macro to create a Vec from a list
- Run with: `cargo test`

**Key pattern:** Separate logic from I/O so logic is testable. `search()` returns results instead of printing — `run()` handles printing.

### 16. Polish

- **`eprintln!`** — like `println!` but writes to stderr. Errors go to stderr so they don't mix with program output when piping.
- **`process::exit(1)`** — exit with non-zero status code to signal failure to the shell.

## Final State of Code

```rust
use std::env;
use std::fs;
use std::process;

enum SearchMode {
    CaseSensitive,
    CaseInsensitive,
}

struct Config {
    query: String,
    filename: String,
    mode: SearchMode,
    line_numbers: bool,
}

impl Config{
    fn new(args: &Vec<String>) -> Result<Config, String> {
        let has_i_flag = args.iter().any(|a| a == "-i");
        let has_n_flag = args.iter().any(|a| a == "-n");

        let non_flags: Vec<&String> = args.iter()
            .skip(1)
            .filter(|a| !a.starts_with("-"))
            .collect();

        match (non_flags.get(0), non_flags.get(1)) {
            (Some(q), Some(f)) => Ok(Config {
                query: q.to_string(),
                filename: f.to_string(),
                mode: if has_i_flag {
                    SearchMode::CaseInsensitive
                } else {
                    SearchMode::CaseSensitive
                },
                line_numbers: has_n_flag,
            }),
            _ => Err(String::from("Usage: rgrep [-i] <query> <filename>"))
        }
    }

    fn search(&self, contents: &str) -> Vec<String> {
        let mut matches: Vec<String> = Vec::new();
        for (index, line) in contents.lines().enumerate() {
            let is_match = match &self.mode {
                SearchMode::CaseSensitive => line.contains(self.query.as_str()),
                SearchMode::CaseInsensitive => {
                    line.to_lowercase().contains(&self.query.to_lowercase())
                }
            };

            if is_match {
                let prefix = if self.line_numbers {
                    format!("{}:", index + 1)
                } else {
                    String::new()
                };
                matches.push(format!("{prefix}{line}"));
            }
        }
        return matches;
    }
}

fn run(config: &Config) -> Result<(), String> {
    let contents = fs::read_to_string(&config.filename)
        .map_err(|e| e.to_string())?;

    for line in config.search(&contents) {
        println!("{line}");
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match Config::new(&args) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };

    if let Err(error) = run(&config) {
        eprintln!("{}", error);
        process::exit(1);
    }
}
```

## Project 1 Complete

All 9 steps finished. `rgrep` is a functional CLI grep clone with:
- Case-sensitive and case-insensitive search (`-i`)
- Line numbers (`-n`)
- Proper error handling (`Result`, `?`, `eprintln!`, exit codes)
- Structured code (`Config` struct, `impl` block, `run()` function)
- Unit tests

## Next Steps (Project 2)

- Data processing tool — iterators, generics, serde, file I/O, testing.
