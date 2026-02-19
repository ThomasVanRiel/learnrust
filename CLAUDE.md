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
**Status:** Complete. All 9 steps done.

### Project 2: Data processing tool (TBD)

**Concepts:** iterators, generics, serde, file I/O, testing.

### Project 3: Web API (TBD)

**Concepts:** async Rust, tokio, axum/actix, database access.

### Project 4: Systems-level project (TBD)

**Concepts:** concurrency, unsafe, FFI, performance.

### Project 5 (Capstone): Tetris TUI

**Concepts:** everything from Projects 1-4 applied together. TUI rendering (ratatui), input handling (crossterm), game loops, concurrency (timer + input threads), state machines, complex data structures.
**Status:** Not started — graduation project after completing Projects 2-4.

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

### Class 02 — 2026-02-18

- Reviewed homework: ownership error when removing `&` from `&args[1]` (E0507 — cannot move out of index of Vec).
- Discussed why you can't move out of a Vec by index (leaves a hole), but can with `.remove()`, `.pop()`, `.into_iter()`.
- Neovim/LazyVim setup: running cargo commands from Neovim (`:!`, `:term`, `Ctrl-/` floating terminal, noice.nvim conflicts).
- Introduced `Result<T, E>` — Rust's error handling (no exceptions, errors are values).
- Introduced `Option<T>` — Rust's replacement for null (`Some`/`None`).
- Replaced `.expect()` on `fs::read_to_string()` with `match` on `Ok`/`Err`.
- Replaced `&args[1]`/`&args[2]` indexing with `.get()` returning `Option`, matched with `match`.
- Taught tuple destructuring in `match`: `(args.get(1), args.get(2))` matched together.
- Taught `_` wildcard as catch-all pattern.
- Completed Step 3 of the incremental plan.
- Started Step 4: created `Config` struct, `parse_config` function returning `Result<Config, String>`.
- Taught implicit returns (expression without semicolon vs statement with semicolon).
- Moved `parse_config` into `impl Config` as `Config::new()` (associated function).
- Added `search(&self, ...)` method — first use of `&self`.
- Taught difference: associated functions (no self, `Type::func()`) vs methods (`&self`, `instance.method()`).
- Step 5: extracted `run()` function, introduced `?` operator, `.map_err()`, `if let`.
- Step 6: defined custom `SearchMode` enum, added `-i` flag support.
- Taught iterator chains: `.iter()`, `.any()`, `.skip()`, `.filter()`, `.collect()`.
- Closures introduced via iterator methods (`|a| a == "-i"`).
- See [class02.md](class02.md) for full notes.

## Class Notes Index

- [Class 01](class01.md) — Setup, tooling, Project 1 kickoff
- [Class 02](class02.md) — Result, Option, match, error handling

## Project 1 Incremental Plan

1. ~~Hardcoded search → basic syntax, `println!`~~ **DONE**
2. ~~Read CLI args → `String`, `Vec`, indexing~~ **DONE**
3. ~~Read a file → `Result`, error handling, `fs::read_to_string`~~ **DONE**
4. ~~Struct for config → structs, impl blocks, methods~~ **DONE**
5. ~~Better error handling → `Result`, `?`, custom errors~~ **DONE**
6. ~~Case-insensitive search → enums, `match`~~ **DONE**
7. ~~Line numbers & formatting → iterators, `enumerate`, closures~~ **DONE**
8. ~~Tests → `#[test]`, `#[cfg(test)]`, integration tests~~ **DONE**
9. ~~Polish → `process::exit`, clean CLI output, edge cases~~ **DONE**

## Current State of Code

`rgrep/src/main.rs` has a `Config` struct with `query`, `filename`, and `mode` (custom `SearchMode` enum) fields. `Config::new()` parses args with flag support (`-i`), separating flags from positional args using iterators. `run()` function uses `?` operator for error propagation. `search()` method matches on `SearchMode` for case-sensitive/insensitive search.

**Concepts Thomas has learned:** `let`, `&str`, `String`, `Vec<String>`, `println!`/`eprintln!`/`{:?}`, `use`, `for`/`if`, `.lines()`, `.contains()`, `.collect()`, `env::args()`, `fs::read_to_string()`, `.expect()`, borrowing with `&`, ownership (three rules), `String` vs `&str`, `Result<T, E>` (`Ok`/`Err`), `Option<T>` (`Some`/`None`), `match`, tuple destructuring, `_` wildcard, `.get()` on Vec, structs, `.to_string()`/`.clone()`, `String::from()`, implicit return (last expression without semicolon), `Result` as return type from functions, `impl` blocks, associated functions vs methods (`Config::new()` vs `config.search()`), `&self`, `?` operator, `.map_err()`, `if let`, custom enums, `.iter()`, `.any()`, `.skip()`, `.filter()`, closures (`|a| ...`), `.to_lowercase()`, `.enumerate()`, `format!()`, `Vec::new()`, `.push()`, `let mut`, `#[test]`, `assert_eq!`, `vec![]`, `#[cfg(test)]`, `mod`, `use super::*`, `process::exit()`.

**Concepts not yet introduced:** traits, generics, lifetimes, modules in depth, closures in depth, `dyn`/`Box`, async.

**Project 1 status:** Complete.

## Notes & Observations

- Student wants project-based learning to explore the Rust ecosystem.
- Prefers running commands himself — don't execute for him, give him the commands to run.
- Asks clarifying questions before proceeding — thorough learner.
- Wants class notes in classXX.md files for future reference.
- Draws on C++ background — analogies to `unique_ptr`, `std::vector`, `const T&` land well.
- Responds well to "why" explanations — not just syntax, but what problem Rust's design solves.
- Uses Neovim with LazyVim. Noice.nvim can interfere with `:!` output — prefer `Ctrl-/` floating terminal or `:term`.
- Also learning Neovim — sprinkle in relevant nvim/LazyVim tips during class when they naturally fit the workflow (e.g., navigation, editing, splits, LSP features). Keep tips short and practical.

## Tutor Habits

- **Reflect and update notes regularly.** At natural breakpoints (after completing a step, before moving to a new topic, or when the student asks), update CLAUDE.md with current progress and update/create the classXX.md file with detailed notes. Don't wait until end of session.
- **Update classXX.md after each reflection.** Class notes should be a complete, standalone reference Thomas can review later — include code snapshots, concept explanations, and key takeaways.
- **Sprinkle Neovim tips.** Occasionally share a useful nvim/LazyVim tip when it's relevant to what Thomas is doing (e.g., navigating errors, jumping to definitions, efficient editing). Keep it brief — one tip at a time, not a lecture.
