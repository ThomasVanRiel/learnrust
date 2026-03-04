# Class 06 — 2026-02-25

## Topics Covered

- Traits: what they are and the contract mental model
- Static dispatch vs dynamic dispatch (`dyn Trait`)
- Polymorphism in Rust vs Java/C#
- Downcasting and why it's rare in idiomatic Rust
- `#[derive(Debug)]` explained — compiler-generated trait impl
- `impl std::fmt::Display for Person` — first hand-written trait impl
- `write!` vs `println!`
- `std::fmt::Result`
- `{}` / `{:?}` connection to `Display` / `Debug`
- `collect::<Result<Vec<T>, _>>()` — collecting fallible iterators
- `Vec::retain()` — in-place filtering
- `sort_by()` with `.cmp()` and `std::cmp::Ordering`
- `Ord` vs `PartialOrd`
- `map_err()` — transforming the error side of `Result`
- Turbofish vs type annotations in method chains

---

## Traits

A trait is a contract: any type implementing it must provide the specified methods.

```rust
trait Greet {
    fn hello(&self) -> String;
}

struct Person { name: String }

impl Greet for Person {
    fn hello(&self) -> String {
        format!("Hi, I'm {}", self.name)
    }
}
```

Key distinction:
- `impl Person { ... }` — methods that belong to `Person` specifically
- `impl Greet for Person { ... }` — `Person` fulfilling a contract

The standard library is full of traits. `PartialOrd`, `Debug`, `Display`, `Ord` are all traits.

---

## Static vs Dynamic Dispatch

**Static dispatch (generics)** — resolved at compile time. The compiler generates a separate version for each concrete type. Zero runtime cost.

```rust
fn compare<T: PartialOrd>(a: T, b: T) -> bool { a < b }
// Compiler generates compare_u32, compare_String, etc.
```

**Dynamic dispatch (`dyn Trait`)** — resolved at runtime via a vtable (like C++ virtual functions). More flexible, small overhead.

```rust
fn print_all(items: &[&dyn Display]) { ... }
```

`dyn Trait` is safe in Rust — the vtable is correct by construction and the borrow checker eliminates dangling pointer UB.

---

## Downcasting

Taking a `dyn Trait` reference and recovering the original concrete type. Rare in idiomatic Rust.

In Rust, downcasting uses the `Any` trait and returns `Option` — never panics (unless you call `.unwrap()`):

```rust
let animal: &dyn Any = &some_dog;
let dog = animal.downcast_ref::<Dog>();  // Option<&Dog>
```

Contrast with Java's `ClassCastException` — Rust forces you to handle the failure case.

If you find yourself downcasting often, it usually means the trait needs more methods rather than recovering the concrete type later.

---

## `#[derive(Debug)]`

The compiler generates an `impl Debug for Person` for you. Roughly equivalent to:

```rust
impl std::fmt::Debug for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Person {{ name: {:?}, age: {:?}, ... }}")
    }
}
```

`derive` is a macro that generates boilerplate trait implementations. Works for `Debug`, `Clone`, `PartialEq`, etc. — but only when all fields also implement that trait.

---

## `impl Display for Person`

`Display` controls what `{}` formatting does. Implemented manually (unlike `Debug` which can be derived).

```rust
impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:<20} {:>4} {:<12} {:>8}",
            self.name, self.age, self.city, self.salary
        )
    }
}
```

- `write!(f, ...)` — same format string syntax as `println!`, but writes into the formatter `f` instead of stdout. `f` could be stdout, a file, a string buffer, a network socket — the caller decides.
- `std::fmt::Result` is a type alias for `Result<(), std::fmt::Error>`.
- `println!` swallows write errors (panics silently). `write!` returns them to you.

The `{}` / `{:?}` connection:
- `{}` calls `Display::fmt`
- `{:?}` calls `Debug::fmt`

Replacing `Person::print()` with `Display` lets you use `println!("{}", record)` — idiomatic and composable with any formatting context.

---

## `collect::<Result<Vec<T>, _>>()`

When an iterator yields `Result<T, E>` items, you can collect directly into `Result<Vec<T>, E>`. It short-circuits on the first error:

```
row 1: Ok(person)  → added to vec
row 2: Ok(person)  → added to vec
row 3: Err(e)      → collect stops, returns Err(e)
row 4: never reached
```

Full chain:

```rust
let mut people: Vec<Person> = reader
    .deserialize()                              // Iterator<Item = Result<Person, csv::Error>>
    .collect::<Result<Vec<Person>, _>>()        // Result<Vec<Person>, csv::Error>
    .map_err(|e| e.to_string())?;              // convert error, propagate with ?
```

The `_` is type inference — "compiler, infer the error type." Not related to error handling.

---

## `map_err()`

Transforms the `Err` side of a `Result`, leaving `Ok` untouched:

```rust
result.map_err(|e| e.to_string())
// Ok(vec)          → Ok(vec)         // unchanged
// Err(csv::Error)  → Err(String)     // error type converted
```

Mirror of `.map()` which transforms the `Ok` side and passes `Err` through.

---

## `Vec::retain()`

In-place filtering. Keeps elements where the closure returns `true`:

```rust
people.retain(|record| match filter.as_str() {
    "name" => op.compare(record.name.to_lowercase(), query.to_lowercase()),
    "age"  => op.compare(record.age, numeric_query.unwrap()),
    _ => false,
});
```

More efficient than `.into_iter().filter(...).collect()` — mutates in place, no new allocation.

### Fallible operations inside closures

`?` doesn't work inside a closure — the closure has its own return type. Solution: lift fallible operations out of the closure, before it runs.

```rust
// Parse upfront where ? works fine (we're in run() which returns Result)
let numeric_query = if filter == "age" || filter == "salary" {
    Some(query.parse::<u32>().map_err(|e| format!("Error: {e}..."))?)
} else {
    None
};

// Closure is now pure bool logic — no error handling needed
people.retain(|record| match filter.as_str() {
    "age" => op.compare(record.age, numeric_query.unwrap()),
    // ...
});
```

`.unwrap()` is safe here — you only call it in the `"age"` arm, and `numeric_query` is only `Some` when `filter == "age"`.

---

## `sort_by()` and `Ord`

`sort_by_key()` requires a single consistent key type — doesn't work when sort field is determined at runtime (could be `String` or `u32`). Use `sort_by()` instead:

```rust
people.sort_by(|a, b| match sort_key.as_str() {
    "name"   => a.name.cmp(&b.name),
    "age"    => a.age.cmp(&b.age),
    "city"   => a.city.cmp(&b.city),
    "salary" => a.salary.cmp(&b.salary),
    _        => std::cmp::Ordering::Equal,
});
```

`cmp()` is from the `Ord` trait — returns `Ordering::Less`, `Ordering::Equal`, or `Ordering::Greater`.

**`Ord` vs `PartialOrd`:**
- `PartialOrd` — allows "I don't know" (used by `f64` for `NaN` comparisons)
- `Ord` — guarantees a definitive answer for every pair (total order)
- Sorting requires `Ord` since every element must be comparable

---

## Turbofish vs Type Annotations in Chains

Two ways to specify a type for `collect`:

```rust
// Turbofish — inline, works in chains
.collect::<Result<Vec<Person>, _>>()

// Type annotation — on the binding
let result: Result<Vec<Person>, _> = iter.collect();
```

In a method chain, you can't annotate intermediate types without breaking the chain into separate statements. Turbofish is the inline equivalent.

For this case, the compiler can't infer what `collect` should produce just from `Vec<Person>` on the final binding — it needs to know whether to collect into `Result<Vec<Person>, _>` or `Vec<Result<Person, _>>` or something else. Turbofish disambiguates.

---

---

## `Copy` vs Non-`Copy` Types

**`Copy` types** — fixed-size, stack-only. Duplicated silently on assignment/pass. Primitives: `u32`, `usize`, `bool`, `f64`, `char`. Tuples/arrays of `Copy` types. References (`&T`).

**Non-`Copy` types** — own heap memory. Duplicating requires allocating new heap memory. `String`, `Vec`, `HashMap`, most structs. These are *moved*, not copied.

```rust
let a: usize = 5;
let b = a;  // copied — both valid

let a = String::from("hello");
let b = a;  // moved — a is no longer valid
```

You can derive `Copy` for your own types if all fields are `Copy`:
```rust
#[derive(Copy, Clone)]
struct Point { x: f64, y: f64 }
```

**`Clone`** is the explicit, potentially expensive duplication (`.clone()`). All `Copy` types are also `Clone`, but not vice versa.

For `Copy` types in `if let`, no `&` needed — just take the value:
```rust
if let Some(limit) = config.limit {  // copies usize, no & needed
    people.truncate(limit);
}
```

---

## `usize` vs `u32`

- **`u32`** — unsigned 32-bit integer, always 4 bytes. Use for domain values: ages, salaries, counts.
- **`usize`** — unsigned integer sized to the platform's pointer size (8 bytes on 64-bit). Use for indexing and sizes: array indices, `Vec` lengths, `truncate()`, etc.

The standard library uses `usize` everywhere for memory-level concepts. No implicit numeric coercion in Rust — `u32` won't silently become `usize`; use `value as usize` if needed.

---

## `--limit` and `Vec::truncate()`

`truncate(n)` keeps only the first `n` elements, dropping the rest in place:

```rust
if let Some(limit) = config.limit {
    people.truncate(limit);
}
```

---

## Multiple `--filter` Flags

`filter: Option<...>` → `filters: Vec<(String, FilterOp, String)>`. Empty vec = no filters — cleaner than `Option<Vec<...>>`.

Parsing all `--filter` flags with an iterator chain:

```rust
let filters: Vec<_> = args
    .iter()
    .enumerate()                                          // (index, arg) pairs
    .filter(|(_, a)| a.as_str() == "--filter")           // find all --filter flags
    .filter_map(|(i, _)| args.get(i + 1))               // get the following arg
    .filter_map(Config::build_filter)                    // parse into filter tuple
    .collect();
```

Note: `.filter_map(|s| Config::build_filter(s))` simplifies to `.filter_map(Config::build_filter)` — a **function pointer**. When a closure just forwards its argument to a function with a matching signature, the closure is redundant. Clippy flags this.

---

## Multi-Filter `retain()` — AND Logic

Loop over filters, calling `retain()` once per filter. Each pass narrows the vec — a record must survive all filters:

```rust
for (filter, op, query) in &config.filters {
    let numeric_query = if filter == "age" || filter == "salary" {
        Some(query.parse::<u32>().map_err(|e| format!("..."))?)
    } else {
        None
    };

    people.retain(|record| match filter.as_str() {
        "name"   => op.compare(record.name.to_lowercase(), query.to_lowercase()),
        "age"    => op.compare(record.age, numeric_query.unwrap()),
        "city"   => op.compare(record.city.to_lowercase(), query.to_lowercase()),
        "salary" => op.compare(record.salary, numeric_query.unwrap()),
        _ => false,
    });
}
```

Multiple `retain()` passes = AND logic. Alternatively, a single `retain()` with `.all()` and `.zip()` achieves the same in one pass, but is more complex. The loop approach is clearer and benefits from shrinking vec size after each selective pass.

---

## `.all()` and `.any()`

- `.all(|x| ...)` — returns `true` if closure returns `true` for **every** element. Short-circuits on first `false`.
- `.any(|x| ...)` — returns `true` if closure returns `true` for **at least one** element. Short-circuits on first `true`.

```rust
vec![2, 4, 6].iter().all(|n| n % 2 == 0)  // true
vec![2, 3, 6].iter().all(|n| n % 2 == 0)  // false — stops at 3
vec![1, 3, 4].iter().any(|n| n % 2 == 0)  // true — stops at 4
```

`.all()` = AND across a collection. `.any()` = OR across a collection.

---

## `.zip()`

Pairs two iterators element-by-element into tuples. Stops when the shorter iterator is exhausted:

```rust
let a = vec![1, 2, 3];
let b = vec!["one", "two", "three"];
a.iter().zip(b.iter())  // (1, "one"), (2, "two"), (3, "three")
```

---

## Final `run()` Structure

```rust
fn run(config: &Config) -> Result<(), String> {
    // Read
    let mut people: Vec<Person> = reader.deserialize()
        .collect::<Result<Vec<Person>, _>>()
        .map_err(|e| e.to_string())?;

    // Filter (AND logic — one retain() pass per filter)
    for (filter, op, query) in &config.filters {
        // pre-parse numeric query before closure...
        people.retain(|record| /* ... */);
    }

    // Sort
    if let Some(sort_key) = &config.sort {
        people.sort_by(|a, b| /* ... */);
    }

    // Limit
    if let Some(limit) = config.limit {
        people.truncate(limit);
    }

    // Print
    for person in people {
        println!("{}", person);
    }

    Ok(())
}
```

Clean read → filter → sort → limit → print pipeline. Collecting into `Vec` first is what makes sorting and limiting possible — streaming row-by-row would prevent both.
