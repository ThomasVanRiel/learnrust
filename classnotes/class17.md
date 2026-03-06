# Class 17 — Iterators in depth

## Plan

- The `Iterator` trait
- Writing your own iterator
- Lazy evaluation
- Iterator adapters: `map`, `filter`, `flat_map`, `zip`, `enumerate`, `take`, `skip`
- Consumers: `collect`, `fold`, `sum`, `count`, `find`, `any`, `all`
- Chaining adapters
- `IntoIterator`

---

## The `Iterator` trait

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

That's the whole contract. Implement `next()` and you get every adapter and consumer for free.

- `Some(item)` — here's the next value
- `None` — iteration is done

---

## Writing your own iterator

```rust
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
```

Now you can use it with all standard iterator methods:

```rust
let sum: u32 = Counter::new(5).sum();           // 15
let doubled: Vec<u32> = Counter::new(5).map(|x| x * 2).collect();
let evens: Vec<u32> = Counter::new(10).filter(|x| x % 2 == 0).collect();
```

---

## Lazy evaluation

Iterators are **lazy** — they do no work until consumed. Adapters like `.map()` and `.filter()` return new iterator types, they don't process anything yet.

```rust
let iter = vec![1, 2, 3, 4, 5]
    .iter()
    .map(|x| {
        println!("mapping {x}");
        x * 2
    })
    .filter(|x| x > &4);

// Nothing printed yet — no work done

let result: Vec<_> = iter.collect();  // NOW it runs
```

This matters for performance — you can chain many adapters with no intermediate allocations.

---

## Iterator adapters (lazy — return new iterators)

### `map` — transform each element

```rust
vec![1, 2, 3].iter().map(|x| x * 2)  // [2, 4, 6]
```

### `filter` — keep elements matching a predicate

```rust
vec![1, 2, 3, 4].iter().filter(|&&x| x % 2 == 0)  // [2, 4]
```

### `flat_map` — map then flatten

```rust
vec!["hello world", "foo bar"]
    .iter()
    .flat_map(|s| s.split_whitespace())
// ["hello", "world", "foo", "bar"]
```

### `zip` — pair two iterators

```rust
let names = vec!["Alice", "Bob"];
let scores = vec![95, 87];
let paired: Vec<_> = names.iter().zip(scores.iter()).collect();
// [("Alice", 95), ("Bob", 87)]
```

### `enumerate` — add index

```rust
for (i, val) in vec!["a", "b", "c"].iter().enumerate() {
    println!("{i}: {val}");
}
```

### `take` and `skip`

```rust
(0..).take(5).collect::<Vec<_>>()   // [0, 1, 2, 3, 4] — infinite range, take 5
vec![1,2,3,4,5].iter().skip(2)      // [3, 4, 5]
```

### `chain` — concatenate two iterators

```rust
let a = vec![1, 2];
let b = vec![3, 4];
a.iter().chain(b.iter()).collect::<Vec<_>>()  // [1, 2, 3, 4]
```

---

## Iterator consumers (eager — drive the iteration)

### `collect` — gather into a collection

```rust
let v: Vec<i32> = (0..5).collect();
let s: HashSet<i32> = vec![1, 1, 2, 3].into_iter().collect();
```

Type annotation required — `collect` can produce many different types.

### `fold` — reduce to a single value with accumulator

```rust
let sum = vec![1, 2, 3, 4].iter().fold(0, |acc, x| acc + x);  // 10
```

`sum`, `product`, `count` are convenience wrappers around `fold`.

### `find` — first element matching predicate

```rust
vec![1, 2, 3, 4].iter().find(|&&x| x > 2)  // Some(3)
```

### `any` and `all`

```rust
vec![1, 2, 3].iter().any(|&x| x > 2)  // true
vec![1, 2, 3].iter().all(|&x| x > 0)  // true
```

Short-circuit — stop early on first match.

### `for_each` — run a closure on each element (no result)

```rust
vec![1, 2, 3].iter().for_each(|x| println!("{x}"));
```

---

## `IntoIterator`

Types that implement `IntoIterator` can be used in `for` loops:

```rust
for x in vec![1, 2, 3] { ... }
// is sugar for:
let mut iter = vec![1, 2, 3].into_iter();
while let Some(x) = iter.next() { ... }
```

`Vec`, arrays, ranges, `HashMap`, and most collections implement `IntoIterator`.

---

## `.iter()` vs `.into_iter()` vs `.iter_mut()`

| Method | Yields | Ownership |
|---|---|---|
| `.iter()` | `&T` | Borrows the collection |
| `.iter_mut()` | `&mut T` | Mutably borrows |
| `.into_iter()` | `T` | Consumes the collection |

```rust
let v = vec![1, 2, 3];
v.iter().for_each(|x| println!("{x}"));   // v still usable
v.into_iter().for_each(|x| println!("{x}")); // v consumed
```

---

## Additional notes from session

### Off-by-one in custom iterators

Increment *after* checking, not before — otherwise the first value is skipped:

```rust
fn next(&mut self) -> Option<Self::Item> {
    if self.count < self.max {
        let val = self.count;
        self.count += 1;  // increment after capturing val
        Some(val)
    } else {
        None
    }
}
```

### Use a `new()` constructor for default starting state

Rust has no default field values in struct literals. Use an associated function:

```rust
impl Fibonacci {
    fn new() -> Self {
        Fibonacci { current: 0, next: 1 }
    }
}
```

### Infinite iterators

An iterator that always returns `Some` is infinite. Pair with `take(n)` or `take_while(|x| ...)` to terminate:

```rust
Fibonacci::new().take(10).collect::<Vec<u64>>()
(0..).filter(|x| x % 3 == 0).take(5).collect::<Vec<_>>()
```

Watch out for overflow — Fibonacci grows fast. `u32` overflows around the 47th term. Use `u64` for headroom, or `take_while` to stop before overflow.

### Lazy evaluation — nothing runs until consumed

Adapters like `.map()` and `.filter()` return new iterator types — no work happens until a consumer (`.collect()`, `.sum()`, `.for_each()` etc.) drives the iteration:

```rust
let iter = Counter::new(5)
    .map(|x| { println!("mapping {x}"); x * 2 })
    .filter(|x| x > &4);
// nothing printed yet
let result: Vec<u32> = iter.collect(); // NOW it runs
```

Each element passes through the full chain before the next is processed. No intermediate allocations.

### `.zip()` stops at the shorter iterator

If one iterator is longer, the extra elements are silently dropped — no error.

### `.split_whitespace()` vs `.split(" ")`

Prefer `.split_whitespace()` for real text — handles multiple spaces, tabs, newlines. `.split(" ")` only splits on a single literal space.

### Inclusive ranges

`1..=5` includes 5. `1..5` excludes 5. `1..=5` reads more naturally for "1 to 5" and works in match patterns too:

```rust
match x {
    1..=5 => println!("small"),
    _ => println!("large"),
}
```

---

## Exercises completed

- `Counter` iterator with `next()`, `.collect()`, `.sum()`, lazy `.map().filter()`
- Infinite range `(0..)` with `.filter().take()`
- `Fibonacci` iterator with `new()` constructor, `u64` to avoid overflow
- `fold` — product of 1..=5
- `flat_map` — sentences to words
- `zip` + `max_by_key` — find highest scorer

---

## Next

- Class 18: Error handling patterns — `thiserror`, `anyhow`
