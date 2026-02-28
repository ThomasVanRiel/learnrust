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
**Status:** Complete. Modules extracted, tests added, `--group-by` with `HashMap` implemented. Moving to lifetimes session then Project 3.

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

### Class 09 — 2026-02-28

- Lifetimes: the dangling reference problem they solve.
- Lifetime elision — compiler infers lifetimes in simple cases (one input → output borrows from it).
- `&s[..end]` — fat pointer (ptr + len) into same memory, no copy. Equivalent to C++ `string_view`.
- Explicit annotations: `fn longer<'a>(a: &'a str, b: &'a str) -> &'a str` — needed when multiple inputs, one output.
- `'a` is a generic parameter, declared in `<>` like `T`, then used in the signature.
- Annotations are constraints (relationships), not durations.
- Lifetimes in structs: `struct Config<'a> { query: &'a str }` — struct can't outlive what it borrows.
- Practical rule: own data in structs (`String` not `&str`), use references in functions. Lifetime-annotated structs are uncommon in real Rust.
- Lifetimes are compile-time only — zero assembly generated.
- See [class09.md](class09.md) for full notes.

### Class 08 — 2026-02-28

- `HashMap<K, V>` introduced — insert, lookup, O(1) average.
- `entry().or_insert(default)` — idiomatic upsert, single lookup.
- `*entry += 1` — dereferencing `&mut T` to mutate through a reference.
- Sorting a `HashMap`: collect entries to `Vec<(&K, &V)>`, then `sort_by_key`.
- `sort_by_key` — sort on a single extracted field (simpler than `sort_by` for this case).
- `BTreeMap` introduced — sorted map, O(log n), vs `HashMap` O(1) unordered.
- Immutable borrows lock mutation: holding `&` refs into a collection prevents `&mut` for the borrow's duration.
- Non-Lexical Lifetimes (NLL): borrows end at last use, not closing brace.
- Added `--group-by <field>` feature to csvtool.
- See [class08.md](class08.md) for full notes.

### Class 07 — 2026-02-25

- Module system: `mod foo;` loads `src/foo.rs`, `mod foo { }` is inline — same result.
- Split `main.rs` into `filter.rs`, `person.rs`, `config.rs`, `main.rs`.
- Visibility: `pub` on struct, fields, and methods. Enum variants always public if enum is `pub`.
- Module paths: `crate::filter::FilterOp` (absolute), `super::` (one level up, like `..`).
- Same folder ≠ same scope — directory layout is irrelevant to the module tree.
- `#[cfg(test)]` / `mod tests` / `use super::*` — unit tests next to code.
- `cargo test <substring>` — runs matching tests. Full path shown: `filter::tests::fn_name`.
- Nested test modules for grouping: `use super::super::*` to reach two levels up.
- `use super::*` idiomatic in tests; `use crate::...` idiomatic everywhere else.
- See [class07.md](class07.md) for full notes.

### Class 06 — 2026-02-25

- Traits introduced: contract mental model, `impl Trait for Type` syntax.
- Static dispatch (generics, zero-cost) vs dynamic dispatch (`dyn Trait`, vtable).
- Polymorphism in Rust vs Java/C# — no inheritance, traits instead.
- Downcasting: recovering concrete type from `dyn Trait` via `Any`, returns `Option` not exception.
- `#[derive(Debug)]` fully explained — compiler generates `impl Debug for Person`.
- Implemented `std::fmt::Display` for `Person` — first hand-written trait impl, replaced `print()` method.
- `write!(f, ...)` vs `println!` — same format strings, different destination; `write!` returns errors.
- `std::fmt::Result` is a type alias for `Result<(), std::fmt::Error>`.
- `{}` calls `Display::fmt`, `{:?}` calls `Debug::fmt`.
- Refactored `run()` to collect all records into `Vec<Person>` first, enabling sort.
- `collect::<Result<Vec<T>, _>>()` — collects fallible iterator, short-circuits on first error.
- `map_err()` — transforms only the `Err` side of `Result`, passes `Ok` through unchanged.
- Fallible operations inside closures: lift the `?` out of the closure by pre-parsing before the closure runs.
- `Vec::retain()` — stable, in-place filtering (vs unstable `extract_if`).
- `sort_by()` with `cmp()` — used when sort key type varies at runtime.
- `Ord` vs `PartialOrd` — total order required for sorting; `f64` uses `PartialOrd` due to `NaN`.
- Turbofish vs type annotation in chains — turbofish annotates intermediate types inline; annotation requires breaking the chain into separate bindings.
- `--limit N` added: `Vec::truncate()`, `usize` vs `u32` (platform pointer size vs fixed domain value).
- `Copy` vs non-`Copy` types: `Copy` types are stack-only and duplicated silently; non-`Copy` own heap memory and are moved. `usize` is `Copy` so `if let Some(limit) = config.limit` copies safely without `&`.
- Multiple `--filter` flags: `filters: Vec<(String, FilterOp, String)>` — empty vec means no filters.
- Parsing multiple flags with an iterator chain: `enumerate()` + `filter()` + `filter_map()` to find all flag positions.
- Redundant closure lint: `|s| Config::build_filter(s)` simplified to `Config::build_filter` (function pointer).
- Multi-filter `retain()`: loop over filters, call `retain()` once per filter — achieves AND logic, each pass narrows the vec.
- `.all()` — returns `true` if closure returns `true` for every element, short-circuits on first `false`. Mirror of `.any()`.
- `.zip()` — pairs two iterators element-by-element into tuples.
- See [class06.md](class06.md) for full notes.

### Class 05 — 2026-02-23

- Implemented `FilterOp` enum with `compare<T: PartialOrd>()` method — first use of generics and trait bounds.
- `for` loops over arrays, with tuple destructuring in the loop variable.
- `str::find()` — substring search returning `Option<usize>`.
- String slicing with range syntax (`&s[..n]`, `&s[n..]`, `a..b`, `a..=b`).
- Operator ordering: check longer operators before shorter ones to avoid ambiguous matches.
- Bug found and fixed: hardcoded `+ 2` offset in string slicing should be `+ op_str.len()`.
- `#[derive(Debug)]` — encountered and used, full explanation deferred to traits session.
- `impl` blocks on enums (not just structs).
- Extracted `run()` function returning `Result<(), String>` — same pattern as rgrep.
- Filter dispatch in `run()`: `match filter.as_str()` with arms for `name`, `age`, `city`, `salary`.
- `return Err(format!(...))` — early return with descriptive error from inside `run()`.
- `format!()` macro — builds an owned `String` like `println!` but returns it instead of printing.
- `main()` wired up with `match Config::new()` and `if let Err(error) = run(&config)`.
- `()` unit type — success value when a function has nothing meaningful to return.
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

## Reference Sheets

- [string.md](string.md) — `String` vs `&str`, conversion methods, when to use which

## Class Notes Index

- [Class 01](class01.md) — Setup, tooling, Project 1 kickoff
- [Class 02](class02.md) — Result, Option, match, error handling
- [Class 03](class03.md) — Project 2 kickoff, csv crate, serde, Config struct, Option patterns
- [Class 04](class04.md) — Config wired into main, filter logic, match on String, parse, turbofish
- [Class 05](class05.md) — FilterOp enum, generics, for loops, str::find, string slicing, bug fix
- [Class 06](class06.md) — Traits, Display, collect into Result<Vec>, retain, sort_by, map_err
- [Class 07](class07.md) — Modules, visibility, pub, crate:: vs super::, unit tests
- [Class 08](class08.md) — HashMap, entry API, BTreeMap, sorting, NLL, --group-by feature
- [Class 09](class09.md) — Lifetimes, elision, explicit annotations, structs with references

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
Split into four modules: `filter.rs` (`FilterOp` enum + `compare<T: PartialOrd>()`), `person.rs` (`Person` struct + `impl Display`), `config.rs` (`Config` struct + `new()` + `build_filter()`), `main.rs` (`main()` + `run()`). Unit tests in `filter.rs` under `#[cfg(test)] mod tests`. `run()` follows a clean read → filter → sort → limit → print/stats pipeline.

**Concepts Thomas has learned:** `let`, `&str`, `String`, `Vec<String>`, `println!`/`eprintln!`/`{:?}`, `use`, `for`/`if`, `.lines()`, `.contains()`, `.collect()`, `env::args()`, `fs::read_to_string()`, `.expect()`, borrowing with `&`, ownership (three rules), `String` vs `&str`, `Result<T, E>` (`Ok`/`Err`), `Option<T>` (`Some`/`None`), `match`, tuple destructuring, `_` wildcard, `.get()` on Vec, structs, `.to_string()`/`.clone()`, `String::from()`, implicit return (last expression without semicolon), `Result` as return type from functions, `impl` blocks, associated functions vs methods (`Config::new()` vs `config.search()`), `&self`, `?` operator, `.map_err()`, `if let`, custom enums, `.iter()`, `.any()`, `.skip()`, `.filter()`, closures (`|a| ...`), `.to_lowercase()`, `.enumerate()`, `format!()`, `Vec::new()`, `.push()`, `let mut`, `#[test]`, `assert_eq!`, `vec![]`, `#[cfg(test)]`, `mod`, `use super::*`, `process::exit()`, external crates (`csv`, `serde`), `#[derive(Deserialize)]`, `#[derive(Debug)]` (used, full explanation deferred), `csv::Reader::from_path()`, `.records()`/`.deserialize()`, format string alignment (`{:<N}`/`{:>N}`), `.repeat()`, `Option<(String, String)>` (tuples in generics), `.position()`, `.find()`, `if let` as expression returning a value, nested `if let`, direct indexing `vec[0]` vs `.get(0)`, field init shorthand, early `return`, `u32`, `for` loops with tuple destructuring, string slicing with range syntax (`&s[..n]`, `&s[n..]`), range syntax (`..`, `..=`, `a..b`), generics `<T>`, trait bounds `<T: Trait>`, `PartialOrd`, `impl` on enums, `Result<(), String>` (unit type `()` as success value), `return Err(...)` for early exit, `format!()` to build error strings.

**Concepts Thomas has also learned (Class 06):** traits (`trait`, `impl Trait for Type`), static vs dynamic dispatch, `dyn Trait`, downcasting via `Any`, `#[derive(Debug)]` as generated trait impl, `impl std::fmt::Display`, `write!(f, ...)`, `std::fmt::Result`, `collect::<Result<Vec<T>, _>>()`, `Vec::retain()`, `sort_by()`, `std::cmp::Ordering`, `Ord` vs `PartialOrd`, `map_err()`, turbofish vs type annotation in chains, `Copy` vs non-`Copy` types, `usize` vs `u32`, `Vec::truncate()`, function pointers (redundant closure), `.all()`, `.zip()`.

**Concepts Thomas has also learned (Class 07):** file-based modules (`mod foo;` → `src/foo.rs`), inline modules (`mod foo { }`), module tree, `pub` on structs/fields/enums/functions, enum variants always public, `crate::` absolute paths, `super::` relative paths, `#[cfg(test)]`, `mod tests`, `use super::*` in test modules, `cargo test <substring>` filtering, nested test modules.

**Concepts Thomas has also learned (Class 08):** `HashMap<K, V>`, `HashMap::new()`, `.entry().or_insert()`, dereferencing `&mut T` with `*`, `sort_by_key`, `BTreeMap` (sorted map, O(log n)), immutable borrows locking mutation, Non-Lexical Lifetimes (NLL).

**Concepts Thomas has also learned (Class 09):** lifetimes (compile-time only, zero runtime cost), lifetime elision, explicit lifetime annotations (`'a`), lifetime parameters in `<>`, `&str` as fat pointer (ptr + len) into existing memory, lifetimes in structs, practical rule (own data in structs).

**Concepts not yet introduced:** closures in depth, `Box<dyn Trait>`, async.

**Project 1 status:** Complete.
**Project 2 status:** Complete. Modules extracted, tests added, `--group-by` with HashMap implemented.

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
