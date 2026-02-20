# Class 03 — 2026-02-20

## Topics Covered

- Project 2 kickoff: `csvtool`
- External crates: `csv`, `serde`
- Serde derive macros and deserialization
- Format string alignment
- `Option<(String, String)>` — tuples as type parameters
- `.position()` on iterators
- `if let` as an expression
- Nested `if let`
- Field init shorthand
- Early `return` from functions

---

## Environment Fix: rust-analyzer on WSL

rust-analyzer wasn't installed by default. Fix:

```
rustup component add rust-analyzer
```

Also: `crates.nvim` is a Neovim plugin, not an LSP server. Use:
```
:LspRestart rust_analyzer
```
not `:LspRestart` (which tries to restart all "servers" including crates.nvim, causing an error).

---

## Project 2: `csvtool`

### Setup

```
cargo new csvtool
cd csvtool
mkdir data
```

Sample data in `data/people.csv`:
```csv
name,age,city,salary
Alice,32,New York,95000
Bob,45,Chicago,82000
Carol,28,New York,71000
Dave,51,Chicago,110000
Eve,36,Seattle,98000
Frank,29,Seattle,67000
Grace,42,New York,105000
```

---

## Step 1: Manual CSV Parsing (Why Crates Exist)

```rust
for line in content.lines() {
    let fields: Vec<&str> = line.split(',').collect();
    println!("{:?}", fields);
}
```

`.split(',')` works like `.lines()` — splits on a character, returns an iterator of `&str` slices.

**The problem:** Add `"Smith, John",38,"New York",91000` to the CSV. The comma inside the quoted name breaks the split — you get four fields instead of the expected row.

CSV has an actual spec (RFC 4180) handling quoted fields, escaped quotes, different delimiters. Writing a correct parser is non-trivial — that's why the `csv` crate exists.

---

## Step 2: The `csv` Crate

Add to `Cargo.toml`:
```toml
[dependencies]
csv = "1"
```

```rust
let mut rdr = match csv::Reader::from_path(filename) {
    Ok(r) => r,
    Err(e) => { eprintln!("Error: {}", e); process::exit(1); }
};

for result in rdr.records() {
    let record = match result {
        Ok(r) => r,
        Err(e) => { eprintln!("Error: {}", e); process::exit(1); }
    };
    println!("{:?}", record);
}
```

Key things:
- `mut` on `rdr` — the reader advances a cursor through the file as you iterate, so it must be mutable
- `rdr.records()` returns an iterator where **each item is a `Result`** — errors can happen per-row, not just on open
- `csv::Reader` used directly without a `use` import — you can always use the full path `crate::Type`

---

## Step 3: Serde — Deserializing into Structs

### Why Serde

Right now each record is a list of strings — you access data by index (`record[0]`). With serde, you can deserialize directly into a typed struct with named fields.

### Setup

```toml
[dependencies]
csv = "1"
serde = { version = "1", features = ["derive"] }
```

`features = ["derive"]` enables the derive macros. Serde itself is a framework; the macros are optional and compiled separately.

### The Code

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Person {
    name: String,
    age: u32,
    city: String,
    salary: u32,
}
```

```rust
for result in rdr.deserialize() {
    let record: Person = match result {
        Ok(r) => r,
        Err(e) => { eprintln!("Error: {}", e); process::exit(1); }
    };
    println!("{:?}", record);
}
```

**What `#[derive(Deserialize)]` does:** Generates the deserialization code at compile time based on field names. The CSV header row (`name,age,city,salary`) is matched against struct field names automatically.

**Comparison to C#/Unity:**
```csharp
// C# — runtime reflection
[JsonProperty("name")]
public string Name { get; set; }
```
```rust
// Rust — compile-time, zero runtime overhead
#[derive(Deserialize)]
struct Person { name: String, ... }
```

Same idea, but Rust's approach generates optimized code at compile time with no reflection.

**Note on the dead_code warning:** The compiler warns that struct fields are never read — it doesn't look inside derive macros. The warning goes away once you access fields directly (e.g., `record.name`).

---

## Format String Alignment

```rust
println!("{:<20} {:>4} {:<12} {:>8}", "NAME", "AGE", "CITY", "SALARY");
println!("{}", "-".repeat(52));
println!("{:<20} {:>4} {:<12} {:>8}", record.name, record.age, record.city, record.salary);
```

- `{:<20}` — left-align in a field 20 characters wide
- `{:>8}` — right-align in a field 8 characters wide
- Same idea as `printf` padding in C

`.repeat(n)` on a string/`&str` — repeats the string n times.

Output:
```
NAME                  AGE  CITY            SALARY
----------------------------------------------------
Alice                  32  New York          95000
Bob                    45  Chicago           82000
```

---

## Step 4: Config Struct with Optional Filter

### The Design

```rust
struct Config {
    filename: String,
    filter: Option<(String, String)>,
}
```

`filter` is `Option<(String, String)>` — a tuple of two strings (field name + value), or nothing. Filter is optional — the user may not pass `--filter`.

**Tuples as type parameters:** You can use any type inside `Option<>`, including tuples. `(String, String)` is the type; `("city", "Seattle")` would be a value.

### Parsing with `.position()`

```rust
args.iter().position(|a| a == "--filter")
```

`.position()` returns `Option<usize>` — the index of the first matching element, or `None`. Unlike `.find()` which returns the element itself, `.position()` gives you the index so you can look at the next element.

### `if let` as an Expression

```rust
let filter = if let Some(pos) = args.iter().position(|a| a.eq("--filter")) {
    if let Some(filter_string) = args.get(pos + 1) {
        let parts: Vec<&str> = filter_string.split('=').take(2).collect();
        if parts.len() == 2 {
            Some((parts[0].to_string(), parts[1].to_string()))
        } else {
            None
        }
    } else {
        None
    }
} else {
    None
};
```

**`if let` as expression:** `if let` can return a value — both branches must return the same type. The last expression in each branch (no semicolon) is the value.

**Nested `if let`:** Each layer unwraps one more `Option`. First finds the position, then gets the next argument, then validates the split result.

**Direct indexing vs `.get()`:**
- `parts.get(0)` — returns `Option<&&str>`, safe even if index is out of bounds
- `parts[0]` — returns `&str` directly, panics if out of bounds

Since we checked `parts.len() == 2`, we know both indices exist — direct indexing is appropriate here.

### Field Init Shorthand

```rust
Ok(Config {
    filename: filename.to_string(),
    filter,   // same as filter: filter,
})
```

When a field name and variable name are the same, you can drop the repetition. Same as ES6/modern JavaScript shorthand.

### Early Return

```rust
let filename = match args.get(1) {
    Some(f) => f.to_string(),
    None => return Err(String::from("Usage: csvtool <file> [--filter heading=query]")),
};
```

`return` exits the function immediately from anywhere — you don't have to wait for the last expression. Useful for early error handling.

### Full `Config::new()` at End of Class

```rust
impl Config {
    fn new(args: &[String]) -> Result<Config, String> {
        let filename = match args.get(1) {
            Some(f) => f.to_string(),
            None => return Err(String::from("Usage: csvtool <file> [--filter heading=query]")),
        };

        let filter = if let Some(pos) = args.iter().position(|a| a.eq("--filter")) {
            if let Some(filter_string) = args.get(pos + 1) {
                let parts: Vec<&str> = filter_string.split('=').take(2).collect();
                if parts.len() == 2 {
                    Some((parts[0].to_string(), parts[1].to_string()))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        match (filename, filter) {
            (Some(filename), filter) => Ok(Config {
                filename: filename.to_string(),
                filter,
            }),
            _ => Err(String::from(
                "Usage: csvtool <file> [--filter heading=query]",
            )),
        }
    }
}
```

**Status at end of class:** `Config::new()` is complete but not yet wired into `main()` — hence the "unused function" warning. Next session: replace `main()`'s inline arg parsing with `Config::new()` and add the filter logic to the print loop.

---

## Key Takeaways

| Concept | Summary |
|---|---|
| `csv` crate | Handles RFC 4180 correctly — quoted fields, embedded commas, etc. |
| `serde` | Compile-time serialization framework — `#[derive(Deserialize)]` generates zero-overhead parsing code |
| `Option<(T, T)>` | Tuples work as generic type parameters |
| `.position()` | Returns `Option<usize>` — the index of a match, not the element |
| `if let` as expression | Both branches return a value; last expression in each branch is the result |
| Field init shorthand | `filter,` instead of `filter: filter,` |
| Early `return` | Exit a function from anywhere, not just the last expression |
| `vec[i]` vs `.get(i)` | Use `[i]` when you've proven the index is valid; use `.get(i)` when unsure |

## Next Session

- Wire `Config::new()` into `main()`
- Implement filter logic in the print loop
- Move toward a `run()` function
- Aggregation: count, sum, average
- Eventually: JSON output with `serde_json`
