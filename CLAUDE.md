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

### Project 2: `csvtool` — CSV data processor

**Concepts:** iterators, generics, serde, file I/O, testing.
**Status:** In progress. `FilterOp` enum implemented with `compare<T: PartialOrd>()` method. All six operators (`==`, `!=`, `>`, `<`, `>=`, `<=`) working. Next: further features TBD.

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

### Class 05 — 2026-02-23

- Implemented `FilterOp` enum with `compare<T: PartialOrd>()` method — first use of generics and trait bounds.
- `for` loops over arrays, with tuple destructuring in the loop variable.
- `str::find()` — substring search returning `Option<usize>`.
- String slicing with range syntax (`&s[..n]`, `&s[n..]`, `a..b`, `a..=b`).
- Operator ordering: check longer operators before shorter ones to avoid ambiguous matches.
- Bug found and fixed: hardcoded `+ 2` offset in string slicing should be `+ op_str.len()`.
- `#[derive(Debug)]` — encountered and used, full explanation deferred to traits session.
- `impl` blocks on enums (not just structs).
- See [class05.md](class05.md) for full notes.

### Class 04 — 2026-02-22

- Wired `Config::new()` into `main()`.
- Extracted `Person::print()` method — avoids repeating the format string.
- Implemented filter logic using `match filter.as_str()` — explained why `String` can't be matched directly against literals.
- Discussed `as_str()` vs `as_ref()` vs `as_deref()`.
- Discussed Rust's lack of runtime reflection — `match` with hardcoded arms is the idiomatic solution.
- `.parse::<u32>()` for converting filter query strings to integers — returns `Result`, error handled with `eprintln!` + `process::exit`.
- Turbofish syntax `::<T>` — explicitly specifying generic types when inference isn't enough.
- `String::from()` vs `.to_string()` — `.to_string()` works on any `Display` type, `String::from()` only on `&str`.
- Teased next step: inequality operators (`>`, `<`, `>=`, `<=`) → will need a custom `FilterOp` enum and updated `Config` struct.
- See [class04.md](class04.md) for full notes.

### Class 03 — 2026-02-20

- Started Project 2: `csvtool` — CSV data processor.
- Fixed rust-analyzer in Neovim/WSL: needed `rustup component add rust-analyzer` explicitly.
- Clarified: `crates.nvim` is a plugin, not an LSP server — use `:LspRestart rust_analyzer`, not `:LspRestart`.
- Created `csvtool` project with `cargo new`, added `data/people.csv` sample dataset.
- Step 1: Manual CSV parsing with `.split(',')` — showed it breaks on quoted fields like `"Smith, John"`.
- Step 2: Added `csv` crate, replaced manual parsing with `csv::Reader::from_path()` + `rdr.records()`.
- Step 3: Added `serde` with `features = ["derive"]`, `#[derive(Debug, Deserialize)]` on `Person` struct.
- Switched to `rdr.deserialize()` — CSV rows deserialized directly into typed `Person` structs.
- Discussed serde vs C#/Unity JSON serialization — same concept, but compile-time, zero-overhead.
- Added formatted table output with format string alignment (`{:<20}`, `{:>8}`) and `.repeat()`.
- Step 4 (in progress): Built `Config` struct with `filename: String` and `filter: Option<(String, String)>`.
- Taught `.position()` on iterators — returns `Option<usize>` with the index of a matching element.
- Practiced `if let Some(x)` with `Option` (extending from class 02).
- Taught `if let` as an expression that returns a value (both branches must return same type).
- Nested `if let` for chained Option unwrapping.
- Direct indexing `vec[0]` vs `.get(0)` — use `[i]` when you've already guaranteed existence.
- Field init shorthand: `filter,` instead of `filter: filter,` (same as JS/ES6).
- Early `return` from functions: `return Err(...)` to exit before the final expression.
- `Config::new()` is fully written but not yet wired into `main()` — next step.
- See [class03.md](class03.md) for full notes.

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
- [Class 03](class03.md) — Project 2 kickoff, csv crate, serde, Config struct, Option patterns
- [Class 04](class04.md) — Config wired into main, filter logic, match on String, parse, turbofish
- [Class 05](class05.md) — FilterOp enum, generics, for loops, str::find, string slicing, bug fix

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

### rgrep (Project 1) — Complete
`rgrep/src/main.rs` has a `Config` struct with `query`, `filename`, and `mode` (custom `SearchMode` enum) fields. `Config::new()` parses args with flag support (`-i`), separating flags from positional args using iterators. `run()` function uses `?` operator for error propagation. `search()` method matches on `SearchMode` for case-sensitive/insensitive search.

### csvtool (Project 2) — In Progress
`csvtool/src/main.rs` has a `Person` struct with `#[derive(Debug, Deserialize)]` and a `print()` method. `FilterOp` enum has six variants (`Eq`, `Ne`, `Gt`, `St`, `Ge`, `Se`) with a `compare<T: PartialOrd>()` method. `Config` struct has `filename: String` and `filter: Option<(String, FilterOp, String)>`. `Config::build_filter()` parses the operator out of the filter string using `.find()` and string slicing, iterating over operators in longest-first order. All six comparison operators are working end-to-end.

**Concepts Thomas has learned:** `let`, `&str`, `String`, `Vec<String>`, `println!`/`eprintln!`/`{:?}`, `use`, `for`/`if`, `.lines()`, `.contains()`, `.collect()`, `env::args()`, `fs::read_to_string()`, `.expect()`, borrowing with `&`, ownership (three rules), `String` vs `&str`, `Result<T, E>` (`Ok`/`Err`), `Option<T>` (`Some`/`None`), `match`, tuple destructuring, `_` wildcard, `.get()` on Vec, structs, `.to_string()`/`.clone()`, `String::from()`, implicit return (last expression without semicolon), `Result` as return type from functions, `impl` blocks, associated functions vs methods (`Config::new()` vs `config.search()`), `&self`, `?` operator, `.map_err()`, `if let`, custom enums, `.iter()`, `.any()`, `.skip()`, `.filter()`, closures (`|a| ...`), `.to_lowercase()`, `.enumerate()`, `format!()`, `Vec::new()`, `.push()`, `let mut`, `#[test]`, `assert_eq!`, `vec![]`, `#[cfg(test)]`, `mod`, `use super::*`, `process::exit()`, external crates (`csv`, `serde`), `#[derive(Deserialize)]`, `#[derive(Debug)]` (used, full explanation deferred), `csv::Reader::from_path()`, `.records()`/`.deserialize()`, format string alignment (`{:<N}`/`{:>N}`), `.repeat()`, `Option<(String, String)>` (tuples in generics), `.position()`, `.find()`, `if let` as expression returning a value, nested `if let`, direct indexing `vec[0]` vs `.get(0)`, field init shorthand, early `return`, `u32`, `for` loops with tuple destructuring, string slicing with range syntax (`&s[..n]`, `&s[n..]`), range syntax (`..`, `..=`, `a..b`), generics `<T>`, trait bounds `<T: Trait>`, `PartialOrd`, `impl` on enums.

**Concepts not yet introduced:** traits, generics, lifetimes, modules in depth, closures in depth, `dyn`/`Box`, async, `HashMap`.

**Project 1 status:** Complete.
**Project 2 status:** In progress.

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
