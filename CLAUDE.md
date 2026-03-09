# Rust Learning Journey - Tutor Notes

## Student Profile

- **Name:** Thomas
- **Background:** Experienced developer. Knows systems languages (C++ — but rusty, ~10 years ago) and managed/GC languages (Java, C#, Python, JS/TS, etc.)
- **Systems concepts:** Comfortable but rusty — understands memory management, pointers, stack/heap, concurrency in theory but hasn't practiced hands-on in a long time.
- **Rust experience:** Complete beginner — never written Rust.
- **Interests:** Broad — CLI tools, web/APIs, systems/low-level, data/automation. All fair game for projects.
- **Learning approach:** Project-based. Explore the Rust ecosystem hands-on.
- **Learning style:** Prefers to execute commands himself rather than having tutor run them. Hands-on learner. Asks good "why" questions (e.g., asked about curl flags before blindly running the install script). Wants class notes stored for future reference.
- **Problem solving:** Let Thomas figure out problems himself first — using the compiler, error messages, and docs. Don't give away the answer immediately. Nudge in the right direction if he asks, but hold back the full solution until he's genuinely stuck. Ask him what he tried and what the compiler said before helping further.

## Curriculum Plan

### Project 1: `rgrep` — mini grep clone (CLI tool)

**Concepts:** cargo, types, ownership, borrowing, structs, enums, pattern matching, error handling, traits, file I/O, iterators, closures, testing.
**Status:** Complete. All 9 steps done.

### Project 2: `csvtool` — CSV data processor

**Concepts:** iterators, generics, serde, file I/O, testing.
**Status:** Complete. Modules extracted, tests added, `--group-by` with `HashMap` implemented. Moving to lifetimes session then Project 3.

### Project 3: `todo-api` — REST API

**Concepts:** async Rust, tokio, axum, serde_json, sqlx, SQLite.
**Status:** Complete. All 5 routes working with SQLite persistence, typed request bodies, and custom error handling.

### Project 4 (Capstone): CHIP-8 Emulator

**Status:** In progress. All 35 opcodes implemented and working. Font sprites preloaded in RAM. Timer thread running at 60Hz via `Arc<Mutex<Timers>>`. `MinifbRenderer` with 10× scaling and keyboard input. Full fetch/decode/execute loop in `main.rs`. Wasm build working in browser via `wasm-pack` and `wasm-bindgen` — same core, JS drives the loop via `requestAnimationFrame`. Tested on both platforms. Next: `NullRenderer` for headless tests, SDL2 renderer (FFI).

**Learning goals:**
- **Emulation fundamentals:** fetch/decode/execute loop, opcode dispatch via `match`, program counter, stack, registers
- **Bit manipulation:** masking, shifting, and extracting nibbles from raw bytes — real low-level byte work
- **Low-level memory:** emulated RAM as `[u8; 4096]`, direct indexing, no abstractions
- **Enums + pattern matching at scale:** all 35 CHIP-8 opcodes decoded into a Rust enum
- **Traits + dynamic dispatch:** `Box<dyn Renderer>` — emulator core is renderer-agnostic
- **Concurrency:** delay and sound timers tick at 60Hz on a separate thread, independent of the CPU loop
- **FFI:** SDL2 renderer as a second backend after minifb — real-world C library interop
- **Testing:** `NullRenderer` for headless opcode unit tests; ROMs as integration tests
- **Module structure:** clean separation of cpu, memory, display, input, timers, renderer
- **WebAssembly:** compile to Wasm with `wasm-pack`, expose to JS via `#[wasm_bindgen]`, render to `<canvas>` — deploy to a real website

**Architecture decision — pluggable renderer:**
The emulator core holds a `Box<dyn Renderer>` and never depends on a concrete backend. The `Renderer` trait exposes three methods:
```rust
pub trait Renderer {
    fn draw(&mut self, display: &[[bool; 64]; 32]);
    fn is_running(&self) -> bool;
    fn pressed_keys(&self) -> Vec<u8>;
}
```
**Phase 1:** `MinifbRenderer` — pure Rust, gets pixels on screen fast.
**Phase 2:** `Sdl2Renderer` via FFI — Thomas knows SDL2 from C++, real-world FFI exposure in context.
**Phase 3:** `WasmRenderer` — compile to WebAssembly with `wasm-pack`, render to HTML5 `<canvas>`, deploy to Thomas's website. Emulator core unchanged — only a new renderer impl needed. JS calls `step()` each frame via `requestAnimationFrame`.
**Testing:** `NullRenderer` — headless, no window, used in unit tests for opcode logic.

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
- See [class01.md](classnotes/class01.md) for full notes.

### Class 19 — 2026-03-08

- Completed all 35 CHIP-8 opcodes: font lookup (FX29), BCD decode (FX33), timer opcodes (FX07/FX15/FX18), input opcodes (EX9E/EXA1/FX0A).
- Font sprites preloaded in `Memory::new()` at `0x000`.
- Timers moved to `Arc<Mutex<Timers>>` shared between CPU and a 60Hz timer thread.
- `MutexGuard` NLL drop behavior — temporary guard released before sleep, not at end of loop.
- `Arc` explained as Rust's `shared_ptr` — atomically reference counted, thread-safe (vs `Rc` which isn't).
- `Box<dyn Renderer>` — `run()` now accepts any renderer, `main()` decides the concrete type.
- `MinifbRenderer` implemented: 10× scaling with `flat_map`/`repeat_n`, keyboard mapping via array position trick.
- `overflowing_add`/`overflowing_sub` for overflow-safe arithmetic — debug mode panics on overflow.
- WSLg setup for GUI apps in WSL2 — `WAYLAND_DISPLAY=""` to force X11 if decoration error occurs.
- Full emulator loop wired in `main.rs` — tested against demo ROMs successfully.
- See [class19.md](classnotes/class19.md) for full notes.

### Class 18 — 2026-03-06

- Error handling patterns: `anyhow`, `thiserror`, when to use which.
- `anyhow::Result<T>` accepts any error type — `?` works across all error types, no `From` impls needed.
- `.context()` / `.with_context()` — adds human-readable message layer, preserves full error chain.
- `thiserror` — `#[error("...")]` generates `Display`, `#[from]` generates `From`. No manual impl blocks.
- `#[from]` lets `?` auto-convert underlying errors into enum variants.
- Tradeoff: `anyhow` gives rich context but no matchable variants; `thiserror` gives structured variants but raw error messages.
- Real projects: `thiserror` for error enums, `anyhow` in application/main code.
- See [class18.md](classnotes/class18.md) for full notes.

### Class 17 — 2026-03-06

- Iterators: the `Iterator` trait, writing your own, lazy evaluation, adapters, consumers.
- `Iterator` trait requires only `next()` — all combinators are default methods.
- Off-by-one trap: increment after capturing the value, not before.
- `new()` constructor idiom for default starting state — Rust has no default field values.
- Infinite iterators — always return `Some`, pair with `take(n)` or `take_while`.
- Fibonacci in Rust — `u32` overflows ~47th term, use `u64`.
- Lazy evaluation confirmed — adapters do no work until a consumer drives iteration.
- `(0..)` infinite range with `.filter().take()`.
- `fold`, `flat_map`, `zip`, `max_by_key` exercises.
- `.split_whitespace()` preferred over `.split(" ")` for real text.
- Inclusive ranges: `1..=5` vs `1..5`, works in match patterns too.
- See [class17.md](classnotes/class17.md) for full notes.

### Class 16 — 2026-03-06

- Closures: capturing by reference, mutable reference, and move.
- `Fn` / `FnMut` / `FnOnce` — the three closure traits and their hierarchy.
- `Fn` captures immutably, `FnMut` mutates captures, `FnOnce` consumes captures.
- `f(f(x))` fails with `FnMut` — two simultaneous `&mut` borrows of `f` (E0499). Fix: `let tmp = f(x); f(tmp)`.
- `move` required when closure outlives its scope — returning from functions, `thread::spawn`.
- `Copy` types in `move` closures are silently copied; non-`Copy` types (e.g. `String`) are moved — only one closure can capture them without `.clone()`.
- `&str` is `Copy` (fat pointer) — multiple closures can capture the same `&str` via `move`, but the closure lifetime is tied to the source string.
- `thread::spawn` requires `FnOnce` + `move`.
- Generic type parameters are just naming conventions — `F`, `T`, etc. can be any valid identifier.
- See [class16.md](classnotes/class16.md) for full notes.

### Class 15 — 2026-03-06

- Traits deep dive: default method implementations, `impl Trait` syntax, `From`/`Into`, `Clone`, `Iterator`.
- Default implementations — define a method body in the trait; implementors get it free unless they override it.
- `Iterator` trait — only requires `next()`, all `.map()` / `.filter()` / `.collect()` / `.any()` etc. are default methods on top of it.
- `impl Trait` in argument position — shorthand for `<T: Trait>`, cleaner for simple cases.
- `impl Trait` in return position — useful mainly for returning closures (anonymous types with no nameable type).
- `From<T>` — defines conversion into a type; `Into` is auto-implemented for free. `?` uses `From` under the hood.
- Tuple destructuring in function parameters: `fn from((r, g, b): (u8, u8, u8))` — type on the whole tuple, destructure the binding.
- `Clone` vs `Copy` — `Clone` is explicit (`.clone()`), `Copy` is implicit. Custom `Clone` only needed for raw pointers; otherwise `#[derive(Clone)]`.
- Mutability lives on the binding, not the type — `let mut x` lets you mutate all of `x`'s fields.
- Scratch project renamed to `class15/`.
- See [class15.md](classnotes/class15.md) for full notes.

### Class 14 — 2026-03-05

- Typed request bodies: `CreateTodo` and `UpdateTodo` structs replacing `serde_json::Value`.
- `Json<CreateTodo>` extractor auto-returns 422 on bad input — no manual validation needed.
- `UpdateTodo` uses `Option<T>` fields — present fields update, absent fields keep current value.
- Custom `ApiError` enum with `IntoResponse` and `From<sqlx::Error>`.
- `From` + `?` — bare `?` auto-converts `sqlx::Error` to `ApiError`, no `map_err` needed.
- `.map(Json)` breaks `?` auto-conversion — split into `let x = query.await?; Ok(Json(x))` instead.
- Exposing DB error messages to clients is a security concern — log internally, return generic message.
- Project 3 complete.
- See [class13.md](classnotes/class13.md) for full notes (classes 13+14 combined).

### Class 13 — 2026-03-04

- Replaced in-memory `Vec` with SQLite via sqlx.
- `SqliteConnectOptions` with `create_if_missing(true)` — creates DB file if absent.
- `SqlitePool` replaces `Arc<Mutex<Vec<Todo>>>` — already thread-safe, no wrapping needed.
- `DATABASE_URL` in `.env` required for compile-time SQL checking.
- `query_as!(Struct, SQL, args)` — maps rows to structs, types must match SQLite defaults (`INTEGER` → `i64`).
- `NOT NULL` in schema → non-optional Rust types. Nullable columns → `Option<T>`.
- `RETURNING *` — get inserted/updated row back without a second query.
- `query!` for statements without row results (DELETE). `rows_affected()` to detect missing rows.
- Extracted `find_todo` helper — reused by `get_todo` and `update_todo`.
- `.ok()` on `Result` converts to `Option`, discarding the error.
- SQL injection — never use `format!()` for queries, always use `?` placeholders.
- `2>&1` in shell — redirects stderr to stdout so compiler errors can be piped.
- See [class13.md](classnotes/class13.md) for full notes.

### Class 12 — 2026-03-04

- Completed Step 2: in-memory todo API, all 5 routes working (GET/POST/PUT/DELETE).
- Walked through code annotations — corrected and explained each one.
- `Arc<Mutex<Vec<Todo>>>` — Arc for shared ownership across threads, Mutex for mutual exclusion.
- `#[derive(...)]` generates trait implementations, not just methods.
- `type Db = ...` is a type alias — purely cosmetic, no new type created.
- Axum extractors are not positional — populated by type, order doesn't matter.
- `.lock()` returns `MutexGuard` — holds lock for its lifetime, releases on drop.
- Mutex poisoning — only fails if another thread panicked while holding the lock.
- Clone in `list_todos` is to escape the lock scope, not because `db` is read-only.
- `IntoResponse` — axum converts `Result<T, E>`, tuples, `StatusCode`, `Json<T>` automatically.
- Axum uses compile-time trait system, not runtime reflection — errors caught at compile time.
- Axum → hyper → Tokio TCP stack explained.
- Combining axum (HTTP) + raw `TcpListener` (streaming) in one app on one Tokio runtime.
- See [class12.md](classnotes/class12.md) for full notes.

### Class 11 — 2026-03-03

- Created `todo-api` project, added tokio + axum + serde dependencies.
- Ran minimal axum Hello World server — `cargo run`, tested with `curl`.
- Clarified `.await` — suspends the task, thread stays free (not "pauses the program").
- Axum vs Tokio: axum is the web layer, tokio is the async runtime/engine.
- Tokio use cases beyond web: HTTP clients, DB, file I/O, TCP servers, message queues, CLI fan-out.
- Futures deep dive: `Future` trait, `poll()`, `Poll::Ready`/`Poll::Pending`, state machines.
- Futures are lazy — calling an async fn does nothing until `.await` or `spawn`.
- Tokio internals: thread pool, event loop, epoll/kqueue, no busy-waiting.
- async vs threads demo: `tokio::time::sleep` (concurrent, ~1s) vs `std::thread::sleep` (blocking, ~3s).
- Core rule: never call blocking functions inside async code — use tokio:: equivalents.
- When not to use async: CPU-bound work → threads or rayon instead.
- `tokio::task::spawn_blocking` — escape hatch for blocking code inside async.
- See [class11.md](classnotes/class11.md) for full notes.

### Class 10 — 2026-02-28

- Project 3 introduced: todo REST API (GET/POST/PUT/DELETE /todos).
- Stack chosen: tokio (async runtime) + axum (web framework) + serde_json + sqlx + SQLite.
- Async Rust introduced: Futures, lazy evaluation, `.await`, cooperative multitasking.
- `#[tokio::main]` — macro that bootstraps the tokio runtime for `main`.
- Rust has async syntax but no built-in runtime — tokio provides the runtime.
- Async vs threads: async is I/O-bound (cheap tasks, cooperative), threads are CPU-bound (expensive, preemptive).
- Axum concepts previewed: handlers, extractors (`Path`, `Json`, `State`), `Router`, `with_state`.
- sqlx concepts previewed: `query_as!` macro, compile-time SQL checking, connection pools.
- See [project3.md](project3.md) for full project reference.

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
- See [class09.md](classnotes/class09.md) for full notes.

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
- See [class08.md](classnotes/class08.md) for full notes.

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
- See [class07.md](classnotes/class07.md) for full notes.

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
- See [class06.md](classnotes/class06.md) for full notes.

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
- See [class05.md](classnotes/class05.md) for full notes.

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
- See [class04.md](classnotes/class04.md) for full notes.

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
- See [class03.md](classnotes/class03.md) for full notes.

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
- See [class02.md](classnotes/class02.md) for full notes.

## Reference Sheets

- [string.md](string.md) — `String` vs `&str`, conversion methods, when to use which

## Class Notes Index

- [Class 01](classnotes/class01.md) — Setup, tooling, Project 1 kickoff
- [Class 02](classnotes/class02.md) — Result, Option, match, error handling
- [Class 03](classnotes/class03.md) — Project 2 kickoff, csv crate, serde, Config struct, Option patterns
- [Class 04](classnotes/class04.md) — Config wired into main, filter logic, match on String, parse, turbofish
- [Class 05](classnotes/class05.md) — FilterOp enum, generics, for loops, str::find, string slicing, bug fix
- [Class 06](classnotes/class06.md) — Traits, Display, collect into Result<Vec>, retain, sort_by, map_err
- [Class 07](classnotes/class07.md) — Modules, visibility, pub, crate:: vs super::, unit tests
- [Class 08](classnotes/class08.md) — HashMap, entry API, BTreeMap, sorting, NLL, --group-by feature
- [Class 09](classnotes/class09.md) — Lifetimes, elision, explicit annotations, structs with references
- [Class 10](project3.md) — Project 3 kickoff, async Rust, tokio, axum, sqlx overview
- [Class 11](classnotes/class11.md) — Hello World server, Futures deep dive, Tokio internals, async vs threads
- [Class 12](classnotes/class12.md) — In-memory todo API, Arc/Mutex, axum extractors, compile-time traits, combining Tokio TCP + Axum
- [Class 13+14](classnotes/class13.md) — SQLite persistence, sqlx, query_as!, typed request bodies, ApiError, From + IntoResponse
- [Class 15](classnotes/class15.md) — Traits deep dive: default impls, `impl Trait`, `From`/`Into`, `Clone`, `Iterator`
- [Class 16](classnotes/class16.md) — Closures: capturing, `Fn`/`FnMut`/`FnOnce`, `move`, returning closures
- [Class 17](classnotes/class17.md) — Iterators: writing your own, lazy evaluation, adapters, consumers
- [Class 18](classnotes/class18.md) — Error handling: `thiserror`, `anyhow`, when to use which
- [Class 19](classnotes/class19.md) — CHIP-8: remaining opcodes, font/BCD, timers thread, Box<dyn Renderer>, MinifbRenderer, input
- [Project 4](chip8.md) — CHIP-8 Emulator: opcode engine, concurrency, pluggable renderer, FFI

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

**Concepts Thomas has also learned (Class 15):** default trait method implementations, `impl Trait` in argument and return position, `From<T>` / `Into` (auto-implemented), tuple destructuring in function parameters, `Clone` vs `Copy` (explicit vs implicit duplication), custom `Clone` only needed for raw pointers, mutability lives on the binding not the type, `Iterator` trait (`next()` only — all combinators are default methods).

**Concepts Thomas has also learned (Class 16):** closures (capturing by `&`, `&mut`, move), `Fn`/`FnMut`/`FnOnce` traits and hierarchy, `move` closures, E0499 (double mutable borrow of `FnMut`), `thread::spawn` with `move`, returning closures with `impl Fn`, `&str` as `Copy` in `move` closures, generic parameter naming conventions.

**Concepts Thomas has also learned (Class 17):** `Iterator` trait (`next()` only), writing custom iterators, lazy evaluation, infinite iterators, `take`/`take_while`, `fold`, `flat_map`, `zip`, `max_by_key`, inclusive ranges (`1..=5`), `split_whitespace`.

**Concepts Thomas has also learned (Class 18):** `anyhow::Result`, `.context()` / `.with_context()`, `thiserror`, `#[error("...")]`, `#[from]`, when to use `anyhow` vs `thiserror`.

**Concepts Thomas has also learned (Class 19):** `Arc` (shared ownership across threads, atomically reference counted), `Mutex`/`MutexGuard` NLL drop behavior, `Arc::clone()`, `Box<dyn Trait>` (dynamic dispatch in practice), `thread::spawn` with `move`, `overflowing_add`/`overflowing_sub`/`wrapping_add`, `repeat_n`, 2D scaling with `flat_map`, `filter_map` with array position trick for key mapping, `wasm-pack`/`wasm-bindgen`, `#[wasm_bindgen]` on structs and impl blocks, `[lib] crate-type = ["cdylib", "rlib"]`, target-specific dependencies (`[target.'cfg(...)'.dependencies]`), `#[cfg(target_arch = "wasm32")]` for conditional compilation, JS `requestAnimationFrame` loop driving Wasm, `Uint8Array` from `Vec<u8>` across Wasm boundary, `getrandom` `wasm_js` feature for Wasm compatibility.

**Concepts not yet introduced:** async.

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
- **Let Thomas struggle productively.** When he hits an error or doesn't know how to proceed, don't give the answer — point him at the compiler output or docs first. Ask "what does the compiler say?" or "look up X in the docs" before explaining. Only give more direct help if he's asked for a nudge and is still stuck. The goal is to build the habit of reading compiler errors and docs, not to get to the answer fastest.
