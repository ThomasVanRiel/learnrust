# Class 01 — Setup & Project 1 Kickoff

**Date:** 2026-02-16

## Topics Covered

### 1. Installing Rust

Installed via rustup — the official Rust toolchain manager:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**What the flags mean:**

- `--proto '=https'` — Only allow HTTPS protocol (the `=` means exclusively, no fallback to HTTP).
- `--tlsv1.2` — Minimum TLS 1.2 (older versions have known vulnerabilities). TLS 1.3 still works — this sets the floor.
- `-s` — Silent (no progress bar).
- `-S` — Show errors (even in silent mode).
- `-f` — Fail fast on HTTP errors (prevents piping a 404 error page into `sh`).
- `| sh` — Pipe the downloaded script into the shell to execute it.

**Safer alternative** (inspect before running):

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup-init.sh
less rustup-init.sh
sh rustup-init.sh
```

**What rustup installs:**

- `rustc` — the Rust compiler
- `cargo` — build tool & package manager
- `rustup` — toolchain version manager

**PATH setup:** The installer adds `. "$HOME/.cargo/env"` to `~/.zshrc`, so new shells automatically have `~/.cargo/bin` in PATH. For the current session: `source "$HOME/.cargo/env"`.

### 2. Cargo Basics

| Command | Purpose |
|---|---|
| `cargo new <name>` | Create a new project |
| `cargo run` | Compile and run |
| `cargo build` | Compile only |
| `cargo check` | Type-check without producing a binary (fastest feedback) |
| `cargo build --release` | Optimized build |

**Project structure created by `cargo new rgrep`:**

```
rgrep/
├── Cargo.toml    # Project metadata & dependencies (like package.json / pom.xml)
├── src/
│   └── main.rs   # Entry point
└── target/       # Build artifacts (created on first build)
```

### 3. Project 1: `rgrep` — Mini Grep Clone

#### What we're building

A simplified version of `grep` — the classic Unix tool that searches for text patterns in files. Usage will look like:

```
rgrep "pattern" file.txt
rgrep "TODO" src/main.rs
rgrep -i "error" log.txt        # case-insensitive
rgrep -n "fn main" src/*.rs     # with line numbers
```

We start with the simplest possible version (search a string in one file, print matching lines) and iteratively add features. By the end, we'll have a genuinely useful tool.

#### Why this project?

- **It's real.** You'll actually use it. Building toy projects is demotivating — this does something.
- **It's bounded.** The core logic is simple (read lines, check if they match, print them), so we can focus on *how Rust wants you to do things* rather than getting lost in domain complexity.
- **It naturally demands every core concept.** You can't build this without hitting ownership, error handling, pattern matching, traits, and iterators. The concepts aren't forced in — they emerge from the problem.
- **It scales.** We can keep adding features (regex, colors, recursion, concurrency) as you level up, without switching projects.

#### Project goals

By the end of Project 1, you will be able to:

1. **Set up and manage** a Rust project with Cargo.
2. **Read and understand** Rust's type system — `String` vs `&str`, `Vec`, `Option`, `Result`.
3. **Explain ownership and borrowing** — why Rust has them, what the compiler is protecting you from, and how to work with (not against) the borrow checker.
4. **Use structs and enums** to model data, and **pattern matching** to work with it.
5. **Handle errors** idiomatically with `Result`, the `?` operator, and custom error types.
6. **Write traits** and understand how they compare to interfaces/abstract classes you already know.
7. **Use iterators and closures** for data transformation (Rust's equivalent of map/filter/reduce).
8. **Write tests** — both unit tests and integration tests with Cargo's built-in test framework.
9. **Read Rust documentation** — know how to navigate docs.rs and understand type signatures.

#### Teaching approach

- **You type, you run, you break things.** I explain concepts and give you code, but you execute it. Making mistakes and reading compiler errors is half the learning.
- **Compiler-driven development.** Rust's compiler has famously good error messages. We'll lean into that — write something, see what the compiler says, and use that as a teaching moment.
- **Incremental complexity.** Each step adds one concept. We don't jump from "Hello, world!" to generics. The progression:
  1. Hardcoded search → introduces basic syntax and `println!`
  2. Read CLI args → `String`, `Vec`, indexing
  3. Read a file → `Result`, error handling, `fs::read_to_string`
  4. Struct for config → structs, impl blocks, methods
  5. Better error handling → `Result`, `?`, custom errors
  6. Case-insensitive search → enums, `match`
  7. Line numbers & formatting → iterators, `enumerate`, closures
  8. Tests → `#[test]`, `#[cfg(test)]`, integration tests
  9. Polish → `process::exit`, clean CLI output, edge cases
- **Compare to what you know.** You come from C++ and managed languages. I'll regularly draw parallels: "this is like a unique_ptr", "this is like an interface", "this replaces try/catch".
- **Explain the *why*, not just the *how*.** Rust makes unusual choices (ownership, no null, no exceptions). Every time we hit one, I'll explain what problem it solves and what the alternative would cost you.

### 4. First Code: Hardcoded Search

Wrote a hardcoded search to learn basic syntax:

```rust
fn main() {
    let poem = "I have a little shadow that goes in and out with me,
And what can be the use of him is more than I can see.
He is very, very like me from the heels up to the head;
And I see him jump before me, when I jump into my bed.";

    let query = "me";

    for line in poem.lines() {
        if line.contains(query) {
            println!("{line}");
        }
    }
}
```

**Key concepts introduced:**
- `let` — declares an immutable variable (opposite default from C++; need `let mut` for mutable).
- `"string literal"` — gives you a `&str` (string slice). A reference to text baked into the binary. Like `const char*` in C++.
- `.lines()` — returns an iterator over lines. Lazy, no allocation.
- `.contains()` — substring search on `&str`.
- `println!("{line}")` — the `!` means it's a macro (not a function). Macros can accept variable arguments, which Rust functions can't. `{line}` inlines the variable (like Python f-strings).

### 5. Reading Command Line Arguments

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
```

Run with: `cargo run -- hello world` (the `--` separates cargo flags from program arguments).

Output: `["target/debug/rgrep", "hello", "world"]` — argument 0 is the program itself, like `argv[0]` in C.

**Key concepts introduced:**
- `use std::env` — import from standard library.
- `env::args()` — iterator over command line arguments.
- `.collect()` — consumes an iterator into a collection. The type annotation tells Rust *which* collection.
- `Vec<String>` — growable array (`std::vector` in C++) of owned strings.
- `{:?}` — debug format specifier (prints internal structure). Regular `{}` uses Display trait, which `Vec` doesn't implement.

### 6. Combining It: Search a File from CLI Args

```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    for line in contents.lines() {
        if line.contains(query.as_str()) {
            println!("{line}");
        }
    }
}
```

Run with: `cargo run -- Rust test.txt`

**Key concepts introduced:**
- `&args[1]` — borrowing. We don't need to own the query, just read it. The `&` creates a reference.
- `fs::read_to_string()` — reads entire file into a `String`. Returns `Result<String, io::Error>`, not a plain `String`.
- `.expect("message")` — unwraps a `Result`. On `Ok`, gives the value. On `Err`, panics with the message. Quick-and-dirty error handling — we'll improve this later.

### 7. Stack vs Heap & Ownership (Conceptual)

#### Stack vs Heap

- **Stack** — fast, automatic, scoped. Fixed size known at compile time. Gone when the function returns.
- **Heap** — dynamic, flexible. Allocated at runtime, lives until freed. Slower (allocation cost, pointer indirection, cache misses).
- **Rust's approach:** No garbage collector, no manual `new`/`delete`. Ownership rules enforced at compile time.

#### Ownership — the three rules

1. **Each value has one owner** — one variable holds it at a time.
2. **When the owner goes out of scope, the value is dropped** (freed automatically).
3. **Ownership can be moved, not copied** (by default).

```rust
let s1 = String::from("hello");   // s1 owns this heap-allocated String
let s2 = s1;                       // ownership MOVES to s2
// println!("{s1}");               // ERROR: s1 is no longer valid
println!("{s2}");                   // fine — s2 is the owner now
```

Move is the default in Rust (unlike C++ where copy is the default). After a move, the original variable is dead — the compiler enforces this. This prevents double-free bugs at compile time.

**C++ analogy:** `String` behaves like `std::unique_ptr` — one owner, non-copyable, automatically freed when scope ends.

#### Borrowing — references without ownership

```rust
let s1 = String::from("hello");
let len = calculate_length(&s1);   // borrow s1
println!("{s1} is {len} chars");   // s1 is still valid!

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

`&s1` creates a reference that doesn't take ownership. Two kinds:

| | Shared `&T` | Mutable `&mut T` |
|---|---|---|
| Can read? | Yes | Yes |
| Can modify? | No | Yes |
| How many at once? | Unlimited | **Exactly one** |
| C++ analogy | `const T&` | `T&` |

**Key rule:** Many shared borrows OR one mutable borrow, never both at the same time. This prevents data races at compile time.

#### `String` vs `&str`

| | `String` | `&str` |
|---|---|---|
| Ownership | **Owns** its data (heap-allocated) | **Borrows** data from somewhere else |
| Mutable | Can grow, shrink, modify | Read-only view |
| C++ analogy | `std::string` | `const std::string_view` |
| Created by | `String::from("hello")`, `.to_string()` | String literals `"hello"`, slicing a `String` |

#### Mental model

- **Ownership** = holding an object. You can give it away (move), then you don't have it.
- **Shared borrow `&`** = lending your book. Many people can read. Nobody can write in it.
- **Mutable borrow `&mut`** = handing someone your notebook to edit. One person at a time, and you can't read while they write.

## Where We Left Off

- Completed Steps 1-2 of the incremental plan (hardcoded search, CLI args + file reading).
- Covered ownership and borrowing conceptually.
- **Current state of `main.rs`:** reads a query and filename from CLI args, searches the file, prints matching lines.
- **Homework:** Try changing `let query = &args[1]` to `let query = args[1]` (remove the `&`) and run `cargo check` to see the compiler error.

## Next Steps (Class 02)

- Step 3: Introduce a `Config` struct to hold query + filename.
- `impl` blocks and methods.
- Start proper error handling with `Result` and the `?` operator.
