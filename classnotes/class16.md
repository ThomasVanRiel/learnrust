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

## Additional notes from session

### `Fn` vs `FnMut` — what the compiler actually checks

A closure that **reads** captured variables implements `Fn`.
A closure that **mutates** captured variables implements `FnMut` (but not `Fn`).

```rust
let name = String::from("Thomas");
let greet = || println!("{}", name);  // Fn — immutable borrow

let mut count = 0;
let inc = || { count += 1; };         // FnMut — mutable borrow
```

Passing a `FnMut` closure where `Fn` is required fails — the closure is too restrictive.

### `f(f(x))` doesn't work with `FnMut`

Calling `f(f(x))` requires two simultaneous `&mut` borrows of `f` — one for the outer call, one to evaluate the argument. That's E0499.

Fix: break into sequential calls:

```rust
fn apply_twice<F: FnMut(i32) -> i32>(mut f: F, x: i32) -> i32 {
    let tmp = f(x);
    f(tmp)
}
```

Note `mut f` in the parameter — required to call a `FnMut`.

### `move` is required when a closure outlives its scope

Returning a closure from a function or spawning a thread — the captured variables must be owned by the closure, not borrowed:

```rust
fn make_multiplier(n: i32) -> impl Fn(i32) -> i32 {
    move |x| n * x  // n is Copy, so move copies it into the closure
}
```

Without `move`, the compiler rejects it because `n` lives in the function's stack frame which is gone after the function returns.

### `Copy` types vs owned types in `move` closures

`i32` is `Copy` — `move` silently copies it. No issue calling the returned closure multiple times or creating multiple closures from the same value.

`String` is not `Copy` — `move` transfers ownership into the first closure. Creating a second closure from the same `String` fails (E0382).

Two solutions:
- `.clone()` — each closure gets independent owned data. Use when the closure needs to outlive the source.
- `&str` — `&str` is `Copy` (fat pointer), so `move` copies the reference. Both closures point at the same string. Use when closures stay local and don't outlive the source.

```rust
fn make_multiplier(n: &str) -> impl Fn(i32) -> String {
    move |x| format!("{}{}", n, x)  // &str is Copy, both closures work
}

let double = make_multiplier(name.as_str());
let triple = make_multiplier(name.as_str());
```

Tradeoff: the closure's lifetime is tied to `name`. Can't return it or send it to a thread without the string living long enough.

### `thread::spawn` requires `move`

The thread may outlive the spawning scope. `move` transfers ownership so the thread carries its own data:

```rust
let name = String::from("Thomas");
let handle = std::thread::spawn(move || {
    println!("{}", name);
});
handle.join().unwrap();
```

`thread::spawn` requires `FnOnce` — the closure runs exactly once on the new thread.

---

## Exercises completed

- `apply_twice` with `Fn` bound, then `FnMut` — discovered E0499 with `f(f(x))`, fixed with `let tmp`
- `thread::spawn` with `move` closure capturing a `String`
- `make_multiplier` returning `impl Fn(i32) -> i32` — explored `i32` (Copy), `String` (move problem), `&str` (Copy reference) variants

---

## Next

- Class 17: Iterators in depth — writing your own, chaining, lazy evaluation
- Class 18: Error handling patterns — `thiserror`, `anyhow`
