# Strings in Rust

## The Two Types

**`String`** — owned, heap-allocated, growable. Owns its memory, dropped when it goes out of scope.

**`&str`** — a borrowed reference (pointer + length) to string data that lives somewhere else. No ownership.

```rust
let s: String = String::from("hello");  // owns the data on the heap
let r: &str = &s;                       // borrows it — points into s's memory
let l: &str = "hello";                  // points into the binary (static memory)
```

String literals like `"hello"` are always `&str` — they're baked into the binary at compile time.

---

## Converting Between Them

### `&str` → `String` (allocates)

```rust
let s: String = "hello".to_string();       // via Display trait
let s: String = String::from("hello");     // explicit constructor
```

Both allocate a new `String` on the heap. Use when you need to own or store the data.

### `String` → `&str` (free)

```rust
let s = String::from("hello");
let r: &str = s.as_str();   // explicit borrow
let r: &str = &s;           // implicit coercion — same thing
```

No allocation — just creates a reference into the existing `String`'s memory.

---

## The Conversion Methods

| Method | From | To | Allocates? |
|---|---|---|---|
| `.to_string()` | anything with `Display` | `String` | yes |
| `String::from()` | `&str` | `String` | yes |
| `.as_str()` | `String` | `&str` | no |
| `&s` / `&s[..]` | `String` | `&str` | no |
| `.clone()` | `String` | `String` | yes |

Note: there is no `.as_string()`.

---

## When to Use Which

**Use `String`** when you need to:
- Own the data (store in a struct, return from a function)
- Mutate or grow the string

**Use `&str`** when you only need to:
- Read the data
- Pass it to a function temporarily

Functions that only read strings should take `&str` — it works with both `String` and `&str`:

```rust
fn greet(name: &str) {
    println!("Hello, {name}!");
}

let owned = String::from("Alice");
greet(&owned);    // String → &str coercion, automatic
greet("Bob");     // &str literal, works directly
```

---

## In Iterator Chains

When collecting unique string values, prefer `&str` to avoid unnecessary allocations:

```rust
use std::collections::HashSet;

// Expensive — clones every name into a new String
let unique: HashSet<String> = people.iter().map(|p| p.name.clone()).collect();

// Free — borrows into existing String data
let unique: HashSet<&str> = people.iter().map(|p| p.name.as_str()).collect();
```

The second version holds references into the `people` vec's memory — no heap allocations.

---

## Common Pitfalls

**Can't return a `&str` that borrows local data:**
```rust
fn make_greeting() -> &str {  // error — what does this borrow?
    let s = String::from("hello");
    s.as_str()  // s is dropped at end of function, reference dangles
}
```
Return `String` instead when the data is created inside the function.

**String literals are `&str`, not `String`:**
```rust
let s = "hello";          // &str
let s = "hello".to_string();  // String
```

**`match` on `String` requires `.as_str()`:**
```rust
let s = String::from("hello");
match s.as_str() {        // can't match String directly against literals
    "hello" => println!("hi"),
    _ => {}
}
```
