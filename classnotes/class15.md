# Class 15 — Traits

## Plan

- What is a trait
- Defining your own trait
- Implementing a trait for a type
- Default implementations
- Trait bounds (`<T: Trait>`)
- Static dispatch vs dynamic dispatch (`dyn Trait`)
- Common standard traits: `Display`, `Debug`, `From`/`Into`, `Clone`, `Iterator`
- `impl Trait` syntax

---

## What is a trait?

A trait is a **contract** — a set of methods a type must implement. Any type that implements the trait can be used wherever the trait is expected.

```rust
trait Greet {
    fn hello(&self) -> String;
}
```

This says: any type implementing `Greet` must have a `hello` method that returns a `String`.

---

## Implementing a trait

```rust
struct Person {
    name: String,
}

impl Greet for Person {
    fn hello(&self) -> String {
        format!("Hello, I'm {}!", self.name)
    }
}
```

Now `Person` satisfies the `Greet` contract. You can call `.hello()` on any `Person`.

---

## Default implementations

Traits can provide default method bodies. Types can override them or use the default:

```rust
trait Greet {
    fn hello(&self) -> String;

    fn goodbye(&self) -> String {
        format!("Goodbye from {}", self.hello())
    }
}
```

`goodbye` has a default — types get it for free unless they override it.

---

## Trait bounds

Generics can require a type to implement a trait:

```rust
fn print_greeting<T: Greet>(item: &T) {
    println!("{}", item.hello());
}
```

`T: Greet` means "T can be any type, as long as it implements Greet". You've seen this with `<T: PartialOrd>` in csvtool's `FilterOp`.

Multiple bounds with `+`:

```rust
fn print<T: Greet + Debug>(item: &T) { ... }
```

---

## Static dispatch vs dynamic dispatch

### Static dispatch — generics (`impl Trait` / `<T: Trait>`)

The compiler generates a separate copy of the function for each concrete type used. Zero runtime cost — resolved at compile time.

```rust
fn print_greeting<T: Greet>(item: &T) {
    println!("{}", item.hello());
}

print_greeting(&person);  // compiler generates print_greeting::<Person>
print_greeting(&robot);   // compiler generates print_greeting::<Robot>
```

### Dynamic dispatch — `dyn Trait`

A single function that works with any type at runtime via a **vtable** (a table of function pointers). Small runtime cost. Needed when the type isn't known at compile time.

```rust
fn print_greeting(item: &dyn Greet) {
    println!("{}", item.hello());
}

let greeters: Vec<Box<dyn Greet>> = vec![
    Box::new(Person { name: "Thomas".to_string() }),
    Box::new(Robot { id: 42 }),
];
```

`Box<dyn Greet>` — heap-allocated value of any type implementing `Greet`. The concrete type is erased, only the trait interface remains.

| | Static (`<T: Trait>`) | Dynamic (`dyn Trait`) |
|---|---|---|
| Resolved | Compile time | Runtime |
| Cost | Zero | Small (vtable lookup) |
| Code size | Larger (one copy per type) | Smaller (one function) |
| Use when | Type known at compile time | Heterogeneous collections, type erasure |

---

## `impl Trait` syntax

Shorthand for trait bounds in function signatures:

```rust
// These are equivalent:
fn print_greeting<T: Greet>(item: &T) { ... }
fn print_greeting(item: &impl Greet) { ... }
```

Also used in return position — when you want to return "something that implements a trait" without naming the type:

```rust
fn make_greeter() -> impl Greet {
    Person { name: "Thomas".to_string() }
}
```

The caller gets back something that implements `Greet` — they can call `.hello()` but don't know the concrete type.

---

## Common standard traits

### `Display` and `Debug`

```rust
use std::fmt;

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
```

- `Display` — human-readable output, used by `{}` in format strings
- `Debug` — developer output, used by `{:?}`. Usually derived: `#[derive(Debug)]`

### `From` and `Into`

`From<T>` defines how to create a type from another type. `Into` is automatically implemented for any type with `From`.

```rust
impl From<&str> for Person {
    fn from(name: &str) -> Person {
        Person { name: name.to_string() }
    }
}

let p = Person::from("Thomas");
let p: Person = "Thomas".into();  // Into works automatically
```

You used this in the todo API: `impl From<sqlx::Error> for ApiError` let `?` auto-convert errors.

### `Clone`

```rust
#[derive(Clone)]
struct Person { name: String }

let a = Person { name: "Thomas".to_string() };
let b = a.clone();  // independent copy
```

### `Iterator`

The most powerful trait in Rust's standard library. A type implementing `Iterator` only needs to define `next()`:

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

All the methods you've used — `.map()`, `.filter()`, `.collect()`, `.find()`, `.any()` — are provided for free by the trait once `next()` is defined.

---

## Exercises

1. Define a `Shape` trait with an `area() -> f64` method
2. Implement it for `Circle` (radius) and `Rectangle` (width, height)
3. Add a default `describe()` method that prints the area
4. Write a function using static dispatch that prints the area of any shape
5. Create a `Vec<Box<dyn Shape>>` with mixed shapes and print all areas

---

## Next

- Class 16: Closures in depth — capturing, `Fn`/`FnMut`/`FnOnce`
- Class 17: Iterators in depth — writing your own, chaining, lazy evaluation
- Class 18: Error handling patterns — `thiserror`, `anyhow`
