# Class 05 — 2026-02-23

## Topics Covered

- Implemented `FilterOp` enum with a `compare()` method
- Generics and trait bounds: `fn compare<T: PartialOrd>`
- `for` loops over arrays
- `str::find()` — substring search returning `Option<usize>`
- String slicing with range syntax (`&s[..n]`, `&s[n..]`)
- `#[derive(Debug)]` — encountered but not yet explained
- Implementing functions on types with `impl`
- Bug hunt: hardcoded offset `+ 2` vs `+ op_str.len()`
- `PartialOrd` on `String` = lexicographic ordering (inequality filters work on string fields too)

---

## `FilterOp` Enum

The set of valid comparison operators is fixed and known at compile time — a perfect fit for an enum:

```rust
#[derive(Debug)]
enum FilterOp {
    Eq,  // ==
    Ne,  // !=
    Gt,  // >
    St,  // < (smaller than)
    Ge,  // >=
    Se,  // <=
}
```

Enums model "one of a fixed set of values". The compiler exhaustively checks all match arms — if you add a new variant later, every match that doesn't have a `_` catch-all will fail to compile, which is a feature: it forces you to handle the new case.

---

## Implementing a Method on an Enum

Just like structs, enums can have `impl` blocks with methods:

```rust
impl FilterOp {
    fn compare<T: std::cmp::PartialOrd>(&self, lhs: T, rhs: T) -> bool {
        match self {
            FilterOp::Eq => lhs == rhs,
            FilterOp::Ne => lhs != rhs,
            FilterOp::Gt => lhs > rhs,
            FilterOp::St => lhs < rhs,
            FilterOp::Ge => lhs >= rhs,
            FilterOp::Se => lhs <= rhs,
        }
    }
}
```

This lets you call `op.compare(record.age, age_query)` instead of a nested `match op { ... }` everywhere you want to do a comparison. Keeps the logic in one place.

### Generics: `<T: PartialOrd>`

`compare` works for any type `T` — `u32`, `String`, `f64`, etc. — as long as that type supports comparison (`>`, `<`, `==`, ...).

The `<T: PartialOrd>` syntax is a **generic with a trait bound**. It reads: "for any type `T` that implements `PartialOrd`." This is how Rust achieves the equivalent of function templates in C++ but with explicit capability requirements.

- `T` is a type parameter — a placeholder filled in at compile time.
- `PartialOrd` is the trait that gives you `>`, `<`, `>=`, `<=`.
- `PartialEq` (implied by `PartialOrd`) gives you `==` and `!=`.

Rust generates a concrete version of `compare` for each type `T` actually used — so `compare::<u32>` and `compare::<String>` are separate compiled functions. Zero runtime overhead.

---

## `for` Loops

You used a `for` loop to iterate over the array of operators in `build_filter`:

```rust
let ops = [
    ("==", FilterOp::Eq),
    ("!=", FilterOp::Ne),
    (">=", FilterOp::Ge),
    ("<=", FilterOp::Se),
    (">",  FilterOp::Gt),
    ("<",  FilterOp::St),
];

for (op_str, op_obj) in ops {
    // ...
}
```

A few things happening here:

- `ops` is an **array** (fixed-size, stack-allocated) of tuples.
- `for x in collection` moves through each element — similar to Java's enhanced for or Python's `for x in`.
- `(op_str, op_obj)` is **destructuring** in the loop variable — unpacks each tuple directly. You've seen this pattern in `match`; it works in `for` too.

**Order matters here.** `">"` must come after `">="` in the array. If `">"` were first, `"age>=21".find(">")` would match at position 3 and you'd treat `"=21"` as the value. By checking longer operators first, you avoid the ambiguity.

---

## `str::find()` — Substring Search

```rust
"age>=21".find(">=")  // Some(3)
"age>=21".find("!")   // None
```

`.find()` searches for a substring (or a char) and returns `Option<usize>` — the byte index where the match starts, or `None` if not found. Same return type as `.position()` which you've seen before; `.find()` is the string-specific version.

Used in `build_filter` to detect which operator appears in the filter string:

```rust
if let Some(op_pos) = filter_string.find(op_str) {
    // op_str found at byte index op_pos
}
```

---

## String Slicing with Ranges

Once you know where the operator is, you can slice out the field and value:

```rust
let field = filter_string[..op_pos].to_string();           // everything before the operator
let value = filter_string[op_pos + op_str.len()..].to_string();  // everything after
```

`..` is Rust's range syntax:

| Syntax | Meaning |
|---|---|
| `..n` | from start up to (not including) `n` |
| `n..` | from `n` to the end |
| `a..b` | from `a` up to (not including) `b` |
| `a..=b` | from `a` up to and including `b` |

String slices produce `&str`, so `.to_string()` converts to an owned `String` for storage in the struct.

---

## The Bug: Hardcoded `+ 2`

The original slice used `op_pos + 2` to skip past the operator:

```rust
filter_string[op_pos + 2..]  // wrong for single-char operators
```

This works for two-character operators (`==`, `!=`, `>=`, `<=`) but breaks for `>` and `<` — it skips 2 characters when only 1 should be skipped, turning `"age>36"` into a query of `"6"` instead of `"36"`.

The fix: use the actual length of the matched operator string:

```rust
filter_string[op_pos + op_str.len()..]  // correct for any operator length
```

Same fix applied to the validity check:

```rust
// Before (wrong):
if filter_string.len() > op_pos + 2 { ... }

// After (correct):
if filter_string.len() > op_pos + op_str.len() { ... }
```

**Lesson:** avoid magic numbers — use the actual length of what you matched. `op_str.len()` is always right; `2` is only accidentally right for some operators.

---

## `#[derive(Debug)]`

You've seen `#[derive(Debug)]` on both `Person` and `FilterOp`. The short story: it auto-generates code that lets you print a value with `{:?}` in format strings.

```rust
println!("{:?}", &config.filter);  // prints the filter tuple for debugging
```

Without `#[derive(Debug)]`, this wouldn't compile — Rust won't let you print a type unless it knows how to format it.

The full story — what `derive` actually is, what traits are, and how this works under the hood — is a topic for when traits come up properly. For now: add `#[derive(Debug)]` to any struct or enum you want to inspect with `{:?}`.

---

## Final `build_filter` Code

```rust
fn build_filter(filter_string: &String) -> Option<(String, FilterOp, String)> {
    let ops = [
        ("==", FilterOp::Eq),
        ("!=", FilterOp::Ne),
        (">=", FilterOp::Ge),
        ("<=", FilterOp::Se),
        (">",  FilterOp::Gt),
        ("<",  FilterOp::St),
    ];
    for (op_str, op_obj) in ops {
        if let Some(op_pos) = filter_string.find(op_str) {
            if filter_string.len() > op_pos + op_str.len() {
                return Some((
                    filter_string[..op_pos].to_string().to_lowercase(),
                    op_obj,
                    filter_string[op_pos + op_str.len()..].to_string().to_lowercase(),
                ));
            } else {
                println!("Filter syntax is col{op_str}query, not {filter_string}");
                return None;
            }
        }
    }
    println!("Filter syntax is col<Op>query, not {filter_string}");
    None
}
```

---

## `PartialOrd` on Strings — Lexicographic Comparison

You tested `city<=seattle` and it worked. That's not a coincidence or a bug — it's `PartialOrd` doing exactly what it's supposed to.

The `compare` method is generic over `T: PartialOrd`. When you filter on `age`, `T` is `u32` and comparison is numeric. When you filter on `city`, `T` is `String` — and `String` implements `PartialOrd` via **lexicographic (alphabetical) ordering**: character by character, by byte value.

Since both sides are lowercased before comparing, `city<=seattle` returns every city that comes alphabetically at or before "seattle":

| City | Lowercased | vs `"seattle"` | Included? |
|---|---|---|---|
| New York | `"new york"` | `'n'` < `'s'` | yes |
| Phoenix | `"phoenix"` | `'p'` < `'s'` | yes |
| Seattle | `"seattle"` | equal | yes |
| Toronto | `"toronto"` | `'t'` > `'s'` | no |

The key insight: **the same `compare` method handles both numeric and string fields without any special casing**. You didn't write separate logic for strings vs numbers — Rust's generics made it work automatically, because both `u32` and `String` implement `PartialOrd`. The semantics differ (numeric order vs alphabetical order) but the code is identical.

This is one of the core payoffs of generics: write the logic once, get correct behaviour for free across all compatible types.

### Why `PartialOrd` and not `Ord`?

You might wonder why it's `Partial`Ord. The `Ord` trait means *every* pair of values is comparable. `PartialOrd` allows comparisons that can produce no result — needed for floating-point numbers, where `NaN` is neither less than, equal to, nor greater than anything. `String` and `u32` are fully ordered (every pair has a result), but they still implement `PartialOrd` because `Ord` requires it as a prerequisite. For now, just know: use `PartialOrd` as your bound when you need `>`, `<`, etc.

---

## Key Takeaways

| Concept | Summary |
|---|---|
| `for (a, b) in collection` | Destructure tuples directly in the loop variable |
| Operator order in search | Check longer operators before shorter ones to avoid ambiguous matches |
| `.find()` | Searches a string for a substring, returns `Option<usize>` |
| String slicing | `&s[..n]` / `&s[n..]` — range syntax for slicing strings and slices |
| Use actual lengths | `op_str.len()` not magic number `2` — avoid hardcoded offsets |
| Generics `<T: Trait>` | Write one function that works for many types, with compile-time checked capabilities |
| `impl` on enums | Enums can have methods just like structs |
| `#[derive(Debug)]` | Auto-generates `{:?}` printing support — full explanation deferred to traits session |
| `PartialOrd` on `String` | Lexicographic ordering — `<=`/`>=` on string fields is alphabetical comparison |
| Generics pay off | One `compare<T: PartialOrd>` handles numbers and strings correctly without special cases |

---

## Project 2 Status

`csvtool` now supports full filter expressions:

```
csvtool data/people.csv --filter age>36
csvtool data/people.csv --filter salary>=80000
csvtool data/people.csv --filter name==Alice
```

`FilterOp` enum with `compare()` method handles all six operators. `build_filter` parses the operator out of the filter string. `Config.filter` stores `Option<(String, FilterOp, String)>`.
