# Rust Exam — Classes 01–07

Work through the sections in order. No looking at notes or code unless a question says you can.
Check your answers with `exam-answers.md` when done.

---

## Part 1 — Concepts (short answer)

**1.1** State Rust's three ownership rules.

**1.2** What is the difference between `String` and `&str`? When would you use each?

**1.3** What does the `?` operator do? What two things must be true for you to use it in a function?

**1.4** What is the difference between an *associated function* and a *method* on a struct? Give the calling syntax for each.

**1.5** What does `#[derive(Debug)]` actually do? How is it different from `impl std::fmt::Display`?

**1.6** Explain the difference between *static dispatch* (generics) and *dynamic dispatch* (`dyn Trait`). What is the cost of each?

**1.7** What is the difference between `Ord` and `PartialOrd`? Why does the standard `.sort()` method require `Ord` rather than just `PartialOrd`? What does this mean for `f64`?

**1.8** What is the difference between `Copy` types and non-`Copy` types? Give one example of each.

**1.9** Why does `mod foo;` in `main.rs` look for a file at `src/foo.rs`? What would `mod foo { ... }` do instead?

**1.10** In a test module, you write `use super::*`. What does `super` refer to and why is this the idiomatic choice?

---

## Part 2 — Code Reading

Read each snippet and answer the question.

---

**2.1** Will this compile? If not, why?

```rust
fn main() {
    let s = String::from("hello");
    let t = s;
    println!("{}", s);
}
```

---

**2.2** What does this function return when called with `words = vec!["foo", "bar", "baz"]` and `target = "bar"`?

```rust
fn find_word(words: &Vec<&str>, target: &str) -> Option<usize> {
    words.iter().position(|w| *w == target)
}
```

---

**2.3** There is a bug in this filter. What is it?

```rust
fn parse_op(s: &str) -> &str {
    if s.starts_with(">=") { return ">="; }
    if s.starts_with(">")  { return ">"; }
    if s.starts_with("<=") { return "<="; }
    if s.starts_with("<")  { return "<"; }
    "="
}
```

Wait — actually, is there a bug here? Justify your answer either way.

---

**2.4** What does this code print?

```rust
let v: Vec<i32> = vec![1, 2, 3, 4, 5];
let result: Vec<i32> = v.iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * 10)
    .collect();
println!("{:?}", result);
```

---

**2.5** What is the type of `result` after this line, and what is its value?

```rust
let result: Result<Vec<u32>, _> = vec!["1", "2", "abc", "4"]
    .iter()
    .map(|s| s.parse::<u32>())
    .collect();
```

---

**2.6** Will this compile? If not, explain the specific problem and describe how to fix it.

```rust
fn run() -> Result<Vec<i32>, String> {
    let strings = vec!["1", "2", "abc", "4"];
    let numbers: Vec<i32> = strings
        .iter()
        .map(|s| {
            s.parse::<i32>().map_err(|e| e.to_string())?
        })
        .collect();
    Ok(numbers)
}
```

---

**2.7** What does `.retain()` do here, and after this call what does `v` contain?

```rust
let mut v = vec![1, 2, 3, 4, 5, 6];
v.retain(|&x| x % 2 != 0);
```

---

**2.8** What does this print, and why?

```rust
#[derive(Debug)]
enum Direction { North, South, East, West }

fn main() {
    let d = Direction::East;
    println!("{:?}", d);
}
```

---

## Part 3 — Fill in the Blank

Fill in the `___` with valid Rust code.

---

**3.1** Make this compile by fixing the one missing annotation:

The function below moves `items`. Rewrite the parameter so it borrows instead, and make the body still work.

```rust
fn double_all(items: Vec<i32>) -> Vec<i32> {
    items.iter().map(|x| x * 2).collect()
}
```

---

**3.2** Complete the trait implementation so `Point` can be printed with `{}`:

```rust
use std::fmt;

struct Point { x: f64, y: f64 }

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ___
    }
}
```

It should produce output like: `(1.5, 2.0)`

---

**3.3** Fill in the match arms so the function works correctly:

```rust
fn describe_option(opt: Option<String>) -> String {
    match opt {
        ___ => format!("Got: {}", s),
        ___ => String::from("Nothing here"),
    }
}
```

---

**3.4** Complete `Config::new()` so it returns `Err` if fewer than 2 args are given, otherwise builds a `Config`:

```rust
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, String> {
        if ___ {
            return Err(String::from("not enough arguments"));
        }
        Ok(Config {
            query: ___,
            filename: ___,
        })
    }
}
```

---

**3.5** Add a generic bound so this compiles:

```rust
fn largest<T___>(list: &[T]) -> &T {
    let mut biggest = &list[0];
    for item in list {
        if item > biggest {
            biggest = item;
        }
    }
    biggest
}
```

---

**3.6** Fill in the test so it passes:

```rust
#[cfg(test)]
mod tests {
    ___

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), ___);
    }
}

fn add(a: i32, b: i32) -> i32 { a + b }
```

---

## Part 4 — Code Writing

Write the complete function or impl block from scratch.

---

**4.1** Write a function `count_matches` that takes a body of text as `&str` and a search pattern as `&str`, and returns the number of lines that contain the pattern as a `usize`.

---

**4.2** Write a function `first_even` that takes a `&[i32]` and returns `Option<i32>` — the first even number, or `None` if there are none. Use a `for` loop.

---

**4.3** Define an enum `Shape` with variants `Circle(f64)` (radius) and `Rectangle(f64, f64)` (width, height). Write a method `area(&self) -> f64` on it.

Use `3.14159` for pi. No imports needed.

---

**4.4** Write a function `parse_number` that takes a `&str`, attempts to parse it as `i32`, and returns:
- `Ok(n)` if it parses
- `Err(String)` with a descriptive message if it doesn't

Do not use `?`. Use a `match` on the parse result.

---

**4.5** Write a function `dedup` that takes a `Vec<String>` and returns a new `Vec<String>` with duplicate strings removed, preserving the first occurrence of each.

Use a `Vec` and the methods you know to track what you've already seen.

---

## Part 5 — Module System

**5.1** You have this file structure:

```
src/
  main.rs
  utils.rs
```

`utils.rs` contains:
```rust
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

Write the two lines needed in `main.rs` to declare the module and call `utils::greet("Thomas")`, storing the result in a variable `msg`.

---

**5.2** Given this module layout, write the `use` statement inside the test module to bring `add` and `subtract` into scope:

```rust
fn add(a: i32, b: i32) -> i32 { a + b }
fn subtract(a: i32, b: i32) -> i32 { a - b }

#[cfg(test)]
mod tests {
    // your use statement here

    #[test]
    fn test_add() { assert_eq!(add(1, 2), 3); }
}
```

---

**5.3** Why does the following fail to compile, and what is the minimal fix?

```rust
mod shapes {
    struct Circle {
        pub radius: f64,
    }
}

fn main() {
    let c = shapes::Circle { radius: 1.0 };
}
```

---

## Part 6 — Harder Problems

These are tougher — worth more in your own self-assessment.

---

**6.1** Explain what `collect::<Result<Vec<T>, E>>()` does that plain `collect::<Vec<...>>()` cannot. When would you use one vs the other?

---

**6.2** You have:

```rust
fn run(config: &Config) -> Result<(), String> {
    let data = std::fs::read_to_string(&config.filename)
        .map_err(|e| e.to_string())?;
    // ... process data ...
    Ok(())
}
```

Explain each step: what does `map_err` do here? What does `?` do? What does `Ok(())` mean?

---

**6.3** Write a trait `Summary` with one method `summarize(&self) -> String`. Then implement it for this struct:

```rust
struct Article {
    title: String,
    author: String,
    body: String,
}
```

`summarize` should return `"<title>, by <author>"`.

---

**6.4** What is wrong with this code? Fix it.

```rust
fn make_greeting(name: String) -> String {
    let greeting = format!("Hello, {}!", name);
    greeting
}

fn main() {
    let name = String::from("Thomas");
    let g1 = make_greeting(name);
    let g2 = make_greeting(name); // use name a second time
    println!("{} {}", g1, g2);
}
```

---

*End of exam. Check your answers in `exam-answers.md`.*
