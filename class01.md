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

## Next Steps

- Read command line arguments in `main.rs`
- First encounter with Rust's `String` vs `&str` and ownership
