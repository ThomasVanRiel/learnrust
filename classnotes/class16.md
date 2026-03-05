# Class 16 — Closures in depth

## Plan

- What is a closure
- Capturing: by reference, by mutable reference, by move
- `Fn`, `FnMut`, `FnOnce` — the three closure traits
- Closures as function parameters and return values
- `move` closures
- When to use closures vs function pointers

---

## What is a closure?

A closure is an anonymous function that can **capture variables from its surrounding scope**.

```rust
let name = "Thomas".to_string();
let greet = |greeting| format!("{}, {}!", greeting, name);  // captures name
println!("{}", greet("Hello"));
```

Unlike regular functions, closures can reference variables from the scope they're defined in.

---

## Capturing

Closures can capture variables three ways:

### By reference (`&T`)

```rust
let name = String::from("Thomas");
let greet = || println!("{}", name);  // borrows name
greet();
println!("{}", name);  // name still usable
```

### By mutable reference (`&mut T`)

```rust
let mut count = 0;
let mut increment = || { count += 1; };
increment();
increment();
// count is now 2
```

### By move (takes ownership)

```rust
let name = String::from("Thomas");
let greet = move || println!("{}", name);  // name moved into closure
greet();
// name is no longer usable here
```

The compiler chooses the least restrictive capture that works. `move` forces ownership transfer.

---

## `Fn`, `FnMut`, `FnOnce`

Every closure implements one or more of these traits:

| Trait | Can call | Captures |
|---|---|---|
| `FnOnce` | Once only | Takes ownership of captured values |
| `FnMut` | Multiple times | Mutably borrows captured values |
| `Fn` | Multiple times | Immutably borrows captured values |

They form a hierarchy: `Fn` implies `FnMut` implies `FnOnce`.

```rust
fn call_once<F: FnOnce()>(f: F) { f(); }
fn call_mut<F: FnMut()>(mut f: F) { f(); f(); }
fn call<F: Fn()>(f: F) { f(); f(); }
```

In practice: use `Fn` in bounds when possible — it's the most flexible. Use `FnOnce` when the closure will only run once (e.g., callbacks, `thread::spawn`).

---

## `move` closures

Forces the closure to take ownership of all captured variables. Required when the closure outlives the scope it was created in — e.g., threads:

```rust
let name = String::from("Thomas");

let handle = std::thread::spawn(move || {
    println!("{}", name);  // name moved into the thread
});

handle.join().unwrap();
```

Without `move`, the thread might outlive `name` — the compiler won't allow it.

---

## Closures as function parameters

You've used this with iterators:

```rust
let numbers = vec![1, 2, 3, 4, 5];
let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect();
```

`.filter()` takes `FnMut(&Self::Item) -> bool`. Any closure matching that signature works.

### Returning closures

Closures have anonymous types — you can't name them. Use `impl Fn` or `Box<dyn Fn>`:

```rust
// impl Fn — zero cost, type must be known at compile time
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

// Box<dyn Fn> — heap allocated, for when the type isn't known at compile time
fn make_adder(n: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + n)
}
```

---

## Function pointers vs closures

Regular functions (not closures) can be used as function pointers `fn(T) -> U`. They implement all three `Fn` traits but cannot capture anything.

```rust
fn double(x: i32) -> i32 { x * 2 }

let nums: Vec<i32> = vec![1, 2, 3].into_iter().map(double).collect();
// same as:
let nums: Vec<i32> = vec![1, 2, 3].into_iter().map(|x| double(x)).collect();
```

You saw this in csvtool — the compiler warned that `|s| Config::build_filter(s)` was a redundant closure and suggested just `Config::build_filter` (function pointer).

Use function pointers when there's nothing to capture. Use closures when you need to capture state.

---

## Exercises

1. Write a closure that captures a multiplier and returns a function that multiplies its input
2. Write a function `apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32`
3. Use `move` to spawn a thread that prints a captured `String`
4. Write a function that returns `impl Fn(i32) -> i32`

---

## Next

- Class 17: Iterators in depth — writing your own, chaining, lazy evaluation
- Class 18: Error handling patterns — `thiserror`, `anyhow`
