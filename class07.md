# Class 07 — 2026-02-25

## Topics Covered

- Rust module system: `mod`, file-based modules, inline modules
- Visibility: `pub` on structs, fields, enums, functions
- Module paths: `crate::`, `super::`, absolute vs relative
- Struct fields vs enum variants — different visibility defaults
- `#[cfg(test)]` and `mod tests` for unit tests
- Running subsets of tests with `cargo test <filter>`
- Grouping tests with nested modules
- `use super::*` vs `use crate::...` in test modules

---

## The Module System

### `mod foo;` vs `mod foo { ... }`

Two syntaxes, same result:

```rust
mod filter;           // load from src/filter.rs
mod filter { ... }    // inline, contents right here
```

Rust doesn't care which you use — the module tree is the same either way. The file system layout is a convention, not a rule.

### The Module Tree

```
crate (src/main.rs)
├── mod filter   → src/filter.rs
├── mod person   → src/person.rs
└── mod config   → src/config.rs
```

`main.rs` is the crate root. All `mod` declarations hang off it (or off other modules). The tree is built at compile time from `mod` declarations, not by scanning the file system.

---

## Visibility

**Everything is private by default.** Unlike Java/C# where types are visible within their package, Rust hides everything until you explicitly say `pub`.

```rust
pub struct Config {       // type visible outside module
    pub filename: String, // field visible outside module
    filters: Vec<...>,    // private — can't be accessed from main.rs
}
```

### Struct fields vs enum variants

| | Default | To expose |
|---|---|---|
| Struct fields | private | `pub field: Type` |
| Enum variants | public (if enum is `pub`) | n/a — always public |

Enum variants are always public because an enum is only useful if you can match on its variants. Hiding individual variants would break matching.

---

## Module Paths

### `crate::` — absolute path from the crate root

Always works regardless of where you are in the tree:

```rust
use crate::filter::FilterOp;  // always refers to the filter module at the root
```

### `super::` — one level up

Like `..` in a file path. Each `super` steps one level up the module tree:

```
crate
└── filter
    └── tests
        └── eq_tests   ← you are here
```

- `super` → `tests`
- `super::super` → `filter`
- `crate::filter` → same as `super::super`, but absolute

### When to use which

- `use super::*` — idiomatic in test modules ("import the thing I'm testing")
- `use crate::...` — everywhere else (clear, survives reorganization)

---

## Splitting csvtool into Modules

Before (all in `main.rs`), after (split into four files):

```
src/
  main.rs      — main(), run()
  config.rs    — Config struct + arg parsing
  person.rs    — Person struct + Display impl
  filter.rs    — FilterOp enum + compare()
```

### `src/filter.rs`

```rust
#[derive(Debug)]
pub enum FilterOp { Eq, Ne, Gt, St, Ge, Se }

impl FilterOp {
    pub fn compare<T: std::cmp::PartialOrd>(&self, rhs: T, lhs: T) -> bool {
        match self {
            FilterOp::Eq => rhs == lhs,
            // ...
        }
    }
}
```

### `src/person.rs`

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub city: String,
    pub salary: u32,
}

impl std::fmt::Display for Person { ... }
```

`use serde::Deserialize` lives here now — the derive macro needs it in scope in the file where the struct is defined.

### `src/config.rs`

```rust
use crate::filter::FilterOp;  // cross-module import — must use crate:: path

pub struct Config {
    pub filename: String,
    pub filters: Vec<(String, FilterOp, String)>,
    pub sort: Option<String>,
    pub limit: Option<usize>,
    pub stats: bool,
}

impl Config {
    pub fn build_filter(filter_string: &String) -> Option<(String, FilterOp, String)> { ... }
    pub fn new(args: &[String]) -> Result<Config, String> { ... }
}
```

### `src/main.rs`

```rust
mod filter;
mod person;
mod config;

use std::collections::HashSet;
use std::env;
use config::Config;
use person::Person;

fn main() { ... }
fn run(config: &Config) -> Result<(), String> { ... }
```

Convention: `mod` declarations first, then `use` statements. Standard library imports before your own.

---

## Unit Tests

Tests live in the same file as the code they test, inside a `#[cfg(test)]` block:

```rust
#[cfg(test)]
mod tests {
    use super::*;   // import everything from the parent module

    #[test]
    fn eq_matches_equal_values() {
        assert!(FilterOp::Eq.compare(5, 5));
    }

    #[test]
    fn eq_rejects_unequal_values() {
        assert!(!FilterOp::Eq.compare(5, 6));
    }
}
```

- `#[cfg(test)]` — only compiled when running `cargo test`, not in normal builds
- `use super::*` — imports from the parent module (the file this is embedded in)
- `cargo test` finds all `#[test]` functions automatically across all files

### Running subsets

`cargo test` uses substring matching on the full module path:

```bash
cargo test filter          # runs all tests in the filter module
cargo test eq_matches      # runs any test with "eq_matches" in the name
cargo test                 # runs everything
```

Output shows full paths:
```
test filter::tests::eq_matches_equal_values ... ok
```

### Grouping tests with nested modules

```rust
#[cfg(test)]
mod tests {
    mod eq_tests {
        use super::super::*;  // two levels up: tests → filter module

        #[test]
        fn matches_equal() { ... }
    }

    mod gt_tests {
        use super::super::*;

        #[test]
        fn matches_greater() { ... }
    }
}
```

Then `cargo test eq_tests` runs only that group.

### `use super::*` vs `use crate::...` in tests

Both work. `super::*` is the universal convention for test modules — it reads as "test the parent." `crate::` is more robust if you reorganize, but `super::*` is idiomatic here.

---

## Key Takeaways

- **Same folder ≠ same scope.** Directory layout is irrelevant to Rust's module system.
- **`pub` is your API surface.** Anything without it is an implementation detail, invisible to outside code.
- **Enum variants are always public** if the enum is `pub`. Struct fields are private by default.
- **`crate::` is an absolute path.** Use it when importing across modules.
- **`#[cfg(test)]` keeps test code out of release builds.** Tests live next to the code they test.
