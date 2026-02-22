# Class 04 — 2026-02-22

## Topics Covered

- Wired `Config::new()` into `main()`
- Filter logic in the print loop using `match`
- `match` on `String` — why you need `.as_str()`
- `as_str()` vs `as_ref()` vs `as_deref()`
- `.parse::<T>()` — converting strings to other types
- `String::from()` vs `.to_string()` — when each works
- Extracted `Person::print()` method
- Next step teaser: inequality operators → custom enum

---

## Wiring `Config::new()` into `main()`

Previously `Config::new()` was written but unused. This session connected it:

```rust
let config = match Config::new(&args) {
    Ok(config) => config,
    Err(error) => {
        eprintln!("{}", error);
        std::process::exit(1);
    }
};
```

Same pattern as `rgrep` — `Config::new()` returns `Result`, match on it and exit cleanly on error.

---

## Extracting `Person::print()`

Rather than repeating the `println!` format string in multiple match arms, extract it as a method:

```rust
impl Person {
    fn print(&self) {
        println!(
            "{:<20} {:>4} {:<12} {:>8}",
            self.name, self.age, self.city, self.salary
        );
    }
}
```

Then call `record.print()` in the loop. Same `&self` pattern as `rgrep`'s `config.search()`.

---

## Filter Logic: `match` on a `String`

The filter field name comes in as a `String` (or `&String`). You can't `match` on a `String` directly with string literal arms like `"name"` — the types don't match.

**Why it fails:**
```rust
match filter.to_string() {  // produces String — doesn't match &str literals
    "name" => ...
}
```

**The fix — `.as_str()`:**
```rust
match filter.as_str() {  // produces &str — matches literals fine
    "name" => ...
    "age" => ...
    "city" => ...
    "salary" => ...
    _ => eprintln!("Filter target not in record headings!"),
}
```

`.as_str()` borrows the contents of a `String` as a `&str`. No allocation, no copy.

### `as_str()` vs `as_ref()` vs `as_deref()`

| Method | What it does | When to use |
|---|---|---|
| `.as_str()` | `&String` → `&str` | Explicit, readable — prefer this |
| `.as_ref()` | Generic conversion via `AsRef<str>` | More flexible, common in generic code |
| `.as_deref()` | `Option<String>` → `Option<&str>` | When you have an `Option<String>` and want `&str` inside |

For now `.as_str()` is the right choice — it says exactly what you mean.

---

## Rust Has No Runtime Reflection

In Python/JS you could do `record[filter_field]` where `filter_field` is a string variable. Rust can't — struct fields are resolved at compile time, not runtime.

The idiomatic solution: `match` with one arm per field. It's verbose for large structs, but with 4 fields it's completely fine. The compiler also enforces you haven't missed any case (or you add `_` explicitly).

---

## `.parse::<T>()` — String to Number

`age` and `salary` are `u32` in the struct but the filter query always arrives as a `String`. Conversion can fail (what if the user types `age=hello`?), so `.parse()` returns a `Result`:

```rust
"age" => match query.parse::<u32>() {
    Ok(age_query) => {
        if record.age == age_query {
            record.print();
        }
    }
    Err(e) => {
        eprintln!("Error: {} while parsing string \"{}\" to u32", e, query);
        std::process::exit(1);
    }
},
```

`::<u32>` is the **turbofish** syntax — tells Rust which type to parse into. Sometimes the compiler can infer it from context; here it can't, so you specify it explicitly.

---

## `String::from()` vs `.to_string()`

| Method | Works on | Notes |
|---|---|---|
| `String::from("hello")` | `&str` only | Direct conversion from string literal |
| `"hello".to_string()` | Any type implementing `Display` | More general — works on numbers, booleans, etc. |

For numeric fields (`u32`), `.to_string()` is the right call. `String::from()` won't compile because `u32` doesn't implement `From<u32>` for `String`.

---

## Final Filter Code

```rust
if let Some((filter, query)) = &config.filter {
    match filter.as_str() {
        "name" => {
            if record.name.to_lowercase() == query.to_lowercase() {
                record.print();
            }
        }
        "age" => match query.parse::<u32>() {
            Ok(age_query) => {
                if record.age == age_query { record.print(); }
            }
            Err(e) => {
                eprintln!("Error: {} while parsing \"{}\" to u32", e, query);
                std::process::exit(1);
            }
        },
        "city" => {
            if record.city.to_lowercase() == query.to_lowercase() {
                record.print();
            }
        }
        "salary" => match query.parse::<u32>() {
            Ok(salary_query) => {
                if record.salary == salary_query { record.print(); }
            }
            Err(e) => {
                eprintln!("Error: {} while parsing \"{}\" to u32", e, query);
                std::process::exit(1);
            }
        },
        _ => eprintln!("Filter target not in record headings!"),
    }
} else {
    record.print();
}
```

---

## Next Step: Inequality Operators

The goal: support `--filter age>21`, `--filter salary>=80000`, etc.

The challenge is in parsing — `Config` currently splits on `=` and stores `(field, value)`. To support operators, you need to also store *which* operator was used.

**Design hint:** The set of valid operators is fixed and known at compile time. That's a perfect fit for a custom **enum**:

```rust
enum FilterOp {
    Eq,   // =
    Gt,   // >
    Lt,   // <
    Gte,  // >=
    Lte,  // <=
}
```

Then `Config.filter` would store `(String, FilterOp, String)` — field, operator, value.

The parsing side needs to detect which operator appears in the filter string before splitting on it.

---

## Key Takeaways

| Concept | Summary |
|---|---|
| `match` on `String` | Use `.as_str()` to get a `&str` for matching against literals |
| No runtime reflection | Struct fields are compile-time — use `match` to map field names to values |
| `.parse::<T>()` | Converts `&str` to any type that implements `FromStr` — returns `Result` |
| Turbofish `::<T>` | Explicitly specifies the generic type when inference isn't enough |
| `.to_string()` vs `String::from()` | `.to_string()` works on any `Display` type; `String::from()` only on `&str` |
| `impl Person { fn print() }` | Extract repeated formatting into a method to avoid duplication |

## Next Session

- Design `FilterOp` enum
- Update `Config` to store `(String, FilterOp, String)`
- Update `Config::new()` parsing to detect operator
- Apply inequality comparisons in the filter loop
