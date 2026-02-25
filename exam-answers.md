# Exam Answers — Classes 01–07

---

## Part 1 — Concepts

**1.1** Rust's three ownership rules:
1. Each value has exactly one owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value is dropped.

---

**1.2** `String` vs `&str`:
- `String` is an owned, heap-allocated, growable string. Use it when you need to own or modify the data.
- `&str` is a borrowed reference to a string slice — it can point into a `String`, a string literal, or any UTF-8 data. Use it in function parameters when you just need to read the string and don't need ownership.
- General rule: function parameters that just read → `&str`. Struct fields or return values that need ownership → `String`.

---

**1.3** The `?` operator:
- If the value is `Ok(x)` (or `Some(x)`), it unwraps and returns `x`.
- If the value is `Err(e)` (or `None`), it returns early from the current function with that error.
- Two requirements: (1) the function must return `Result` (or `Option`), and (2) the error type must be compatible (or convertible).

---

**1.4** Associated functions vs methods:
- Associated function: no `self` parameter. Called as `Type::function()`. Example: `Config::new()`.
- Method: has `&self`, `&mut self`, or `self` as first parameter. Called on an instance: `config.search()`.

---

**1.5** `#[derive(Debug)]`:
- The compiler automatically generates an `impl Debug for YourType` that prints a structural representation of the type (field names and values).
- `impl std::fmt::Display` you write yourself — it's for human-readable output, used by `{}` in format strings. `Debug` is used by `{:?}`. You control the exact output with `Display`; `Debug` is auto-generated.

---

**1.6** Static vs dynamic dispatch:
- **Static dispatch** (generics, `<T: Trait>`): the compiler generates a separate copy of the function for each concrete type at compile time. Zero runtime cost, but larger binary.
- **Dynamic dispatch** (`dyn Trait`): a vtable pointer is used at runtime to find the right method. There is a small runtime cost (pointer indirection) but allows mixing different concrete types in the same collection.

---

**1.7** `Ord` vs `PartialOrd`:
- `PartialOrd` — a comparison that can return "no result". Used by `f64` because `NaN` is not less than, equal to, or greater than any value — comparisons involving `NaN` produce no definitive ordering.
- `Ord` — a *total* ordering: every pair of values has a definitive answer (`Less`, `Equal`, or `Greater`). `u32` and `String` implement `Ord`.
- `.sort()` requires `Ord` because it must produce a definitive result for every pair of elements. If any comparison could fail (return nothing), sorting couldn't complete.
- For `f64`: you cannot call `.sort()` on a `Vec<f64>`. Use `.sort_by()` with a manual comparison:
  ```rust
  v.sort_by(|a, b| a.partial_cmp(b).unwrap());
  ```
  The `.unwrap()` panics if a `NaN` is present — that's the trade-off you accept.

---

**1.8** `Copy` vs non-`Copy`:
- `Copy` types are stored entirely on the stack. When assigned or passed, they are silently duplicated. Example: `i32`, `f64`, `bool`, `usize`, `char`.
- Non-`Copy` types own heap memory (or other resources). Assignment or passing moves ownership. Example: `String`, `Vec<T>`.

---

**1.9** `mod foo;` vs `mod foo { }`:
- `mod foo;` is a declaration that tells the compiler to load the module from the file `src/foo.rs` (or `src/foo/mod.rs`). The file provides the body.
- `mod foo { ... }` is an *inline* module — the body is written directly in the file. Both produce an identical module in the module tree; the difference is just where the source lives.

---

**1.10** `use super::*` in tests:
- `super` refers to the parent module — the module that contains the `mod tests { }` block, which is typically the module your code is in.
- It's idiomatic because tests live right next to the code they test and need access to all of it, including private functions. `use super::*` is the conventional shorthand for that.

---

## Part 2 — Code Reading

**2.1** Does not compile.
- `let t = s;` moves the `String` out of `s`. After this line, `s` is no longer valid.
- `println!("{}", s);` then tries to use a moved value → compile error E0382.
- Fix: `let t = s.clone();` or borrow instead: `let t = &s;`.

---

**2.2** Returns `Some(1)`.
- `"bar"` is at index 1 in the vector. `.position()` returns `Option<usize>`, so the result is `Some(1)`.

---

**2.3** There is no bug here — this version is correct.
- The longer operators (`>=`, `<=`) are checked before the shorter ones (`>`, `<`). This is the right order.
- The bug would exist if `>` were checked before `>=` — a string like `">= 5"` would incorrectly match `>` first. But as written, the ordering is correct.

---

**2.4** Prints: `[20, 40]`
- `.filter()` keeps only even numbers: `[2, 4]`.
- `.map()` multiplies each by 10: `[20, 40]`.

---

**2.5** Type is `Result<Vec<u32>, std::num::ParseIntError>`. Value is `Err(...)`.
- When collecting an iterator of `Result` into `Result<Vec<T>, E>`, if any element is `Err`, the whole collection short-circuits and returns that `Err`.
- `"abc".parse::<u32>()` fails, so `result` is `Err(invalid digit found in string)`.

---

**2.6** Does not compile.
- The problem: `?` is used inside a closure (`|s| { ... }`). The `?` operator returns early from the enclosing function — but the closure is not the function. The closure's return type is not `Result`, so `?` has nothing to return from.
- The compiler error will say something like: `` `?` cannot be used in a closure that returns `i32` ``.
- Fix: collect into `Result<Vec<i32>, _>` and lift the error handling outside the closure:
  ```rust
  fn run() -> Result<Vec<i32>, String> {
      let strings = vec!["1", "2", "abc", "4"];
      let numbers: Vec<i32> = strings
          .iter()
          .map(|s| s.parse::<i32>().map_err(|e| e.to_string()))
          .collect::<Result<Vec<i32>, _>>()?;
      Ok(numbers)
  }
  ```
  Now each `.map()` produces a `Result`, and `collect::<Result<Vec<i32>, _>>()` short-circuits on the first error. The `?` is back in `run()` where it belongs.

---

**2.7** `.retain()` keeps only elements for which the closure returns `true`. It filters in-place.
- After the call, `v` contains `[1, 3, 5]` — only the odd numbers.

---

**2.8** Prints: `East`
- `#[derive(Debug)]` generates a `Debug` implementation that prints the variant name.
- `{:?}` calls `Debug::fmt`. Output is `East`.

---

## Part 3 — Fill in the Blank

**3.1** Change the parameter to borrow a slice:

```rust
fn double_all(items: &[i32]) -> Vec<i32> {
    items.iter().map(|x| x * 2).collect()
}
```

Callers pass `&my_vec` or `&my_array`. The body is unchanged — `.iter()` works on both owned `Vec` and borrowed slices.

---

**3.2**

```rust
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

---

**3.3**

```rust
fn describe_option(opt: Option<String>) -> String {
    match opt {
        Some(s) => format!("Got: {}", s),
        None    => String::from("Nothing here"),
    }
}
```

`Some(s)` destructures the `Option`, binding the inner `String` to `s` so it can be used in the format string.

---

**3.4**

```rust
impl Config {
    fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(String::from("not enough arguments"));
        }
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}
```

Note: `args[0]` is the program name, so query is at index 1 and filename at index 2. The length check is `< 3` (need at least 3 elements). `.clone()` is needed because `args` is borrowed.

---

**3.5**

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
```

The bound `PartialOrd` is needed to use `>` between `T` values.

---

**3.6**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

---

## Part 4 — Code Writing

**4.1**

```rust
fn count_matches(text: &str, pattern: &str) -> usize {
    text.lines().filter(|line| line.contains(pattern)).count()
}
```

`.lines()` gives an iterator over lines. `.filter()` keeps only lines that contain the pattern. `.count()` consumes the iterator and returns how many elements it had.

---

**4.2**

```rust
fn first_even(nums: &[i32]) -> Option<i32> {
    for &n in nums {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}
```

`for &n in nums` destructures the reference — each `n` is an owned `i32`. Returning `Some(n)` exits the function immediately on the first match. If the loop finishes without finding one, the function falls through to `None`.

---

**4.3**

```rust
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => 3.14159 * r * r,
            Shape::Rectangle(w, h) => w * h,
        }
    }
}
```

---

**4.4**

```rust
fn parse_number(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(e) => Err(format!("Failed to parse '{}': {}", s, e)),
    }
}
```

---

**4.5**

```rust
fn dedup(v: Vec<String>) -> Vec<String> {
    let mut seen: Vec<String> = Vec::new();
    for s in v {
        if !seen.contains(&s) {
            seen.push(s);
        }
    }
    seen
}
```

`seen` tracks which strings have been encountered. `.contains()` checks membership. Because `v` is consumed by the `for` loop, no clone is needed for the strings that go into `seen`.

---

## Part 5 — Module System

**5.1**

```rust
mod utils;

fn main() {
    let msg = utils::greet("Thomas");
}
```

Or with a `use` to shorten the call:
```rust
mod utils;
use utils::greet;

fn main() {
    let msg = greet("Thomas");
}
```

---

**5.2**

```rust
use super::*;
```

This brings everything in the parent module (where `add` and `subtract` live) into the test module's scope.

---

**5.3** Two problems:
1. `Circle` struct itself is not public — `shapes::Circle` is inaccessible outside the module.
2. Even with a `pub` struct, its fields are private by default — struct literal construction requires all fields to be accessible.

Minimal fix — add `pub` to the struct:

```rust
mod shapes {
    pub struct Circle {
        pub radius: f64,
    }
}
```

Both `pub struct Circle` and `pub radius` are needed. `pub` on a struct does not automatically make its fields public.

---

## Part 6 — Harder Problems

**6.1** `collect::<Result<Vec<T>, E>>()`:
- When you have an iterator of `Result<T, E>` and collect into `Result<Vec<T>, E>`, it short-circuits on the first `Err` and returns it immediately.
- Plain `collect::<Vec<Result<T, E>>>()` just accumulates all results, including errors, into a vec — no short-circuiting.
- Use `Result<Vec<T>, E>` when you want all-or-nothing: if any element fails, the whole operation fails. Use `Vec<Result<T, E>>` when you want to process each result individually.

---

**6.2** Step by step:
- `std::fs::read_to_string(&config.filename)` returns `Result<String, std::io::Error>`.
- `.map_err(|e| e.to_string())` transforms only the `Err` side: converts `std::io::Error` → `String`, so the type becomes `Result<String, String>`. `Ok` values pass through unchanged.
- `?` unwraps the `Ok(contents)` into `data`, or if it's `Err(msg)` it returns early from `run()` with that `Err(msg)`.
- `Ok(())` returns success. `()` is the unit type — the function's success value is "nothing meaningful", just the signal that it completed without error.

---

**6.3**

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    author: String,
    body: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.title, self.author)
    }
}
```

---

**6.4** Problem: `name` is moved into `make_greeting` on the first call. The second call tries to use a moved value — compile error.

Fix — change `make_greeting` to borrow `&str` instead of taking ownership:

```rust
fn make_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    let name = String::from("Thomas");
    let g1 = make_greeting(&name);
    let g2 = make_greeting(&name);
    println!("{} {}", g1, g2);
}
```

`&name` coerces from `&String` to `&str` automatically. The function no longer consumes `name`, so it remains valid for both calls.

---

*Good luck reviewing! If anything is unclear, bring it to the next class.*
