# Class 09 — Lifetimes

## What problem lifetimes solve

Dangling references — pointers to memory that no longer exists. Classic C++ bug:

```cpp
std::string* get_name() {
    std::string name = "Alice";
    return &name; // returns pointer to stack memory
} // name destroyed here — pointer is now dangling
```

Rust prevents this at compile time via lifetimes. Every reference has a lifetime — a label for how long the referenced data is guaranteed to live. The compiler checks that no reference outlives the data it points to.

---

## Lifetime elision — why you haven't needed `'a` yet

This compiles with no lifetime annotations:

```rust
fn first_word(s: &str) -> &str {
    let end = s.find(' ').unwrap_or(s.len());
    &s[..end]
}
```

The compiler applies **elision rules** — simple patterns it recognises automatically. One input reference → output borrows from that input. So the compiler silently treats this as:

```rust
fn first_word<'a>(s: &'a str) -> &'a str { ... }
```

You didn't write it — elision handled it.

### What `&s[..end]` actually is

`&s[..end]` doesn't copy any characters. It creates a new `&str` — a fat pointer (pointer + length) — pointing into the **same memory** as `s`, just with a smaller length:

```
s:       [ ptr → "hello world", len: 11 ]
&s[..5]: [ ptr → "hello world", len: 5  ]
                  ^^^^^
                  same memory, shorter view
```

No allocation, no copy. This is why `&str` is cheap — always a view into existing memory. (Equivalent to C++'s `std::string_view`.)

---

## Explicit lifetime annotations

When the compiler can't infer which input a returned reference borrows from:

```rust
fn longer(a: &str, b: &str) -> &str {  // ERROR: missing lifetime specifier
    if a.len() > b.len() { a } else { b }
}
```

Two inputs, one output — compiler can't know if the output borrows from `a` or `b`. Fix:

```rust
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
```

### Why `<'a>` appears in the function signature

`'a` is a generic parameter, just like `T` in `fn foo<T>`. It must be declared in `<>` before it can be used. Same rule:

```rust
fn compare<T: PartialOrd>(a: T, b: T) -> bool { ... }  // T declared, then used
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str { ... } // 'a declared, then used
```

### What the annotation means

`'a` doesn't specify a duration — it's a constraint: "both inputs must live at least as long as `'a`, and the output lives for `'a`." In practice: the output can't outlive either input.

Valid:
```rust
let s1 = String::from("long string");
{
    let s2 = String::from("xy");
    let result = longer(s1.as_str(), s2.as_str());
    println!("{result}"); // fine — result used before s2 drops
}
```

Invalid:
```rust
let s1 = String::from("long string");
let result;
{
    let s2 = String::from("xy");
    result = longer(s1.as_str(), s2.as_str());
} // s2 dropped here
println!("{result}"); // ERROR: s2 doesn't live long enough
```

The compiler doesn't know which branch `longer` takes at runtime — it assumes worst case.

---

## Lifetimes in structs

Structs can hold references, but must annotate them:

```rust
struct Config<'a> {
    query: &'a str,
}
```

This says: "a `Config` cannot outlive the string it borrows `query` from."

Valid:
```rust
let text = String::from("hello world");
let config = Config { query: &text };
println!("{}", config.query);
// config and text both drop here — fine
```

Invalid:
```rust
let config;
{
    let text = String::from("hello world");
    config = Config { query: &text };
} // text dropped here
println!("{}", config.query); // ERROR: text doesn't live long enough
```

---

## The practical rule

**Structs holding references are uncommon in real Rust.** Lifetime annotations on structs propagate everywhere and make things painful. In practice:

- Structs own their data: `String` not `&str`, `Vec<T>` not `&[T]`
- Use references in function signatures when you need to borrow without taking ownership
- Reach for `&str` in structs only when you have a specific performance reason

This is why `Person` in csvtool has `String` fields — no lifetime annotations needed anywhere.

---

## Key takeaways

- Lifetimes are a **compile-time only** concept — they generate zero assembly. References are just pointers at runtime.
- Elision handles the common cases — you rarely need to write `'a` explicitly.
- Explicit annotations are needed when: multiple input references, one output reference, compiler can't determine which input the output borrows from.
- Lifetime annotations are **constraints**, not durations — they describe relationships between references, not time.
- Own your data in structs. Use references in functions.
- There's more (`'static`, subtyping, higher-ranked trait bounds) but it's advanced and rarely needed day-to-day.
