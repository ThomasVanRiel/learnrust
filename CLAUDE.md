# Rust Learning Journey - Tutor Notes

## Student Profile

- **Name:** Thomas
- **Background:** Experienced developer. Knows systems languages (C++ — but rusty, ~10 years ago) and managed/GC languages (Java, C#, Python, JS/TS, etc.)
- **Systems concepts:** Comfortable but rusty — understands memory management, pointers, stack/heap, concurrency in theory but hasn't practiced hands-on in a long time.
- **Rust experience:** Complete beginner — never written Rust.
- **Interests:** Broad — CLI tools, web/APIs, systems/low-level, data/automation. All fair game for projects.
- **Learning approach:** Project-based. Explore the Rust ecosystem hands-on.
- **Learning style:** Prefers to execute commands himself rather than having tutor run them. Hands-on learner. Asks good "why" questions (e.g., asked about curl flags before blindly running the install script). Wants class notes stored for future reference.

## Curriculum Plan

### Project 1: `rgrep` — mini grep clone (CLI tool)

**Concepts:** cargo, types, ownership, borrowing, structs, enums, pattern matching, error handling, traits, file I/O, iterators, closures, testing.
**Status:** In progress — completed Steps 1-2 (hardcoded search, CLI args + file reading). Next: structs, error handling.

### Project 2: Data processing tool (TBD)

**Concepts:** iterators, generics, serde, file I/O, testing.

### Project 3: Web API (TBD)

**Concepts:** async Rust, tokio, axum/actix, database access.

### Project 4: Systems-level project (TBD)

**Concepts:** concurrency, unsafe, FFI, performance.

## Session Log

### Class 01 — 2026-02-16

- Initial meeting. Gathered background info.
- Installed Rust via rustup (1.93.1).
- Discussed curl install command flags in detail (--proto, --tlsv1.2, -sSf).
- Discussed how .cargo/env gets sourced in .zshrc.
- Introduced Project 1: `rgrep`.
- Covered cargo basics: `cargo new`, `cargo run`, `cargo build`, `cargo check`, `cargo build --release`.
- Explained Cargo.toml and project structure.
- Wrote first Rust code: hardcoded search with `let`, `&str`, `.lines()`, `.contains()`, `println!`.
- Read CLI args with `env::args()`, `Vec<String>`, `.collect()`, debug printing `{:?}`.
- Combined CLI args + file reading with `fs::read_to_string()`, `.expect()`.
- Deep dive on stack vs heap, ownership (three rules), borrowing (`&T` vs `&mut T`), `String` vs `&str`.
- Completed Steps 1-2 of the incremental plan.
- See [class01.md](class01.md) for full notes.

## Class Notes Index

- [Class 01](class01.md) — Setup, tooling, Project 1 kickoff

## Project 1 Incremental Plan

1. ~~Hardcoded search → basic syntax, `println!`~~ **DONE**
2. ~~Read CLI args → `String`, `Vec`, indexing~~ **DONE**
3. Read a file → `Result`, error handling, `fs::read_to_string` — **UP NEXT**
4. Struct for config → structs, impl blocks, methods
5. Better error handling → `Result`, `?`, custom errors
6. Case-insensitive search → enums, `match`
7. Line numbers & formatting → iterators, `enumerate`, closures
8. Tests → `#[test]`, `#[cfg(test)]`, integration tests
9. Polish → `process::exit`, clean CLI output, edge cases

## Current State of Code

`rgrep/src/main.rs` reads a query and filename from CLI args, searches the file line by line, and prints matching lines. Uses `.expect()` for error handling (intentionally naive — will be improved in Steps 3-5).

**Concepts Thomas has learned:** `let`, `&str`, `String`, `Vec<String>`, `println!`/`{:?}`, `use`, `for`/`if`, `.lines()`, `.contains()`, `.collect()`, `env::args()`, `fs::read_to_string()`, `.expect()`, borrowing with `&`, ownership (three rules), `String` vs `&str`.

**Concepts not yet introduced:** structs, enums, `match`, `impl`, traits, `Result`/`Option` (used `.expect()` but hasn't unwrapped manually), `?` operator, closures, iterators beyond `.lines()`, testing, modules.

**Homework assigned:** Remove the `&` from `let query = &args[1]` and run `cargo check` to see the ownership error.

## Notes & Observations

- Student wants project-based learning to explore the Rust ecosystem.
- Prefers running commands himself — don't execute for him, give him the commands to run.
- Asks clarifying questions before proceeding — thorough learner.
- Wants class notes in classXX.md files for future reference.
- Draws on C++ background — analogies to `unique_ptr`, `std::vector`, `const T&` land well.
- Responds well to "why" explanations — not just syntax, but what problem Rust's design solves.
