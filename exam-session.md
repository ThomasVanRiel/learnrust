# Exam Session — 2026-02-25

Progress through `exam.md` question by question.

---

## Intermediary Grade (Parts 1–2 completed)

| Part | Score |
|---|---|
| Part 1 — Concepts (8 scored, 2 deferred) | ~60% |
| Part 2 — Code Reading | ~84% |
| **Overall** | **~72%** |

**Strengths:** iterators, collect, retain, move semantics in practice — applied well under pressure.
**Weak spots:** ownership rules wording, `?` vs `.unwrap()`, Copy/non-Copy reasoning (size vs heap ownership).
**Deferred:** static vs dynamic dispatch (Q1.6), Ord vs PartialOrd (Q1.7) — neither has been used in a project yet.

---

## Part 1 — Concepts

### Q1.1 — Three ownership rules

**Thomas's answer:**
1. Every object has one owner. It can be moved.
2. It can be borrowed (read only) unlimited times.
3. It can be borrowed (edit, or mutable) one at a time.

**Feedback:** Rule 1 correct. Rules 2 and 3 are actually the *borrowing rules*, not the ownership rules — mixed up two related but separate concepts. The missing rule 3 is the key one: when the owner goes out of scope, the value is dropped.

**Correct answer:**
1. Each value has exactly one owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value is dropped.

**Borrowing rules** (separate, also real):
- Any number of shared `&T` borrows at once, OR exactly one `&mut T` borrow — never both simultaneously.

**Score: 1/3** — ownership rule 1 correct; rules 2–3 wrong (confused with borrowing rules).

---

### Q1.2 — `String` vs `&str`

**Thomas's answer:**
String is on the heap and can be altered. &str is static. &str points to a specific string in memory and is hence read only.

**Feedback:** Correct that `String` is heap-allocated and mutable, and that `&str` is read-only. The error: `&str` is not "static" — that's `&'static str`, which applies only to string literals baked into the binary. `&str` in general can point into any string data, including a heap-allocated `String`. The key distinction is ownership: `String` owns its data; `&str` borrows from somewhere else.

**Correct answer:**
- `String` — owned, heap-allocated, growable/mutable.
- `&str` — a borrowed reference to string data (can point into a `String`, a literal, or any UTF-8 data). Read-only.
- Use `&str` for function parameters that just read. Use `String` for struct fields or anywhere you need ownership.

**Score: partial** — core idea right, "static" claim wrong, ownership framing missing.

---

### Q1.3 — The `?` operator

**Thomas's answer:**
It returns the Ok() value or panics on error. The function must return Result<_,Err>.

**Feedback:** Described `.unwrap()`, not `?`. Critical distinction: `?` does NOT panic — it returns early from the function with the `Err` value. Panicking is `.unwrap()`. Got one of the two conditions (function must return `Result`) but missed the second (error types must be compatible), and the syntax was slightly off (`Result<T, E>` not `Result<_,Err>`).

**Correct answer:**
- `Ok(x)` → unwraps to `x`, execution continues.
- `Err(e)` → returns early from the function with `Err(e)`. No panic.
- Condition 1: function must return `Result<T, E>`.
- Condition 2: the error type must be compatible (why `.map_err()` is often needed before `?`).

**Score: 1/3** — early return behaviour wrong (said panic), one of two conditions given.

---

### Q1.4 — Associated functions vs methods

**Thomas's answer:**
`fn function(arg) {...}` called as `struct::function(arg)`. `fn method(&self, arg) {...}` called as `struct_obj.method(arg)`.

**Feedback:** Correct. Minor note: `struct` is a keyword — in practice the type name is capitalised (e.g. `Config::new()`, `config.search()`), but the concept is fully right.

**Score: 3/3**

---

### Q1.5 — `#[derive(Debug)]` vs `impl Display`

**Thomas's answer:**
It implements the `{:?}` formatter based on the fields. Display is user defined and implements the `{}` format code.

**Feedback:** Correct. Addendum worth knowing: `derive` works at compile time — the compiler generates `impl Debug for YourType`, and it only works if all fields also implement `Debug`.

**Score: 3/3**

---

### Q1.6 — Static vs dynamic dispatch

**Thomas's answer:**
Static dispatch generates a generic function at compile time with appropriate types. Dynamic dispatch does it at runtime. Not confident on `dyn Trait` — haven't used it.

**Feedback:** Static dispatch description correct. Formal term: *monomorphization* — compiler stamps out a concrete version per type used (e.g. `compare::<u32>`, `compare::<String>`). Dynamic dispatch deferred to next class — Thomas learns by doing and hasn't used `dyn Trait` yet.

**Score: deferred**

---

### Q1.7 — `Ord` vs `PartialOrd`

**Thomas's answer:**
Doesn't know — deferred to next class.

**Correct answer:**
- `PartialOrd` — comparison can produce no result. Used by `f64` because `NaN` is not less than, equal to, or greater than anything.
- `Ord` — total ordering: every pair produces a definitive `Less`, `Equal`, or `Greater`. `u32`, `String`, `i32` implement `Ord`.
- `.sort()` requires `Ord` because sorting needs a definitive answer for every comparison.
- `Vec<f64>` can't use `.sort()` — use `.sort_by(|a, b| a.partial_cmp(b).unwrap())` instead.

**Score: 0/3 — revisit**

---

## Part 2 — Code Reading

### Q2.1 — Move error

**Thomas's answer:**
`s` was moved to `t` so it will not compile.

**Feedback:** Correct. Compiler error E0382: use of moved value. Fix: `s.clone()` or `let t = &s`.

**Score: 3/3**

### Q2.8 — `#[derive(Debug)]` on enum

**Thomas's answer:**
Prints `Direction::East`. Debug formatter is compiler-generated at compile time.

**Feedback:** "Why" correct. Output wrong — prints just `East`, not `Direction::East`. Fieldless enum variants print only the variant name. With fields (e.g. `East(f64)`) it would print `East(1.5)`.

**Score: 2/3**

---

### Q2.7 — `retain()`

**Thomas's answer:**
Keeps odd values. `v` will be `[1, 3, 5]`.

**Feedback:** Correct.

**Score: 3/3**

---

### Q2.6 — `?` inside a closure

**Thomas's answer:**
`.map_err` should be outside the closure. `?` can be removed because `.map` halts on error. The value can't be unwrapped as `Ok()` is expected.

**Feedback:** Right instinct (`?` is the problem) but reasoning imprecise. Specific issue: `?` can't be used inside a closure — the closure's return type is `i32`, not `Result`, so `?` has nowhere to return from. `.map()` does not halt on error — it produces `Result` values. `collect::<Result<Vec<i32>, _>>()` is what short-circuits. Fix: remove `?` from the closure, let `.map()` produce `Result`s, then `collect::<Result<...>>()` followed by `?` in `run()` where it's valid.

**Score: partial** — identified the symptom, reasoning imprecise.

---

### Q2.5 — `collect` into `Result`

**Thomas's answer:**
`Err` containing a parsing error — "invalid digit found".

**Feedback:** Correct. Type is `Result<Vec<u32>, ParseIntError>`. Short-circuits on `"abc"`, never processes `"4"`.

**Score: 3/3**

---

### Q2.4 — Iterator chain output

**Thomas's answer:**
`[20, 40]`. Asked why `|&&x|` in filter vs `|&x|` in map.

**Feedback:** Correct. Good follow-up question. `v.iter()` yields `&i32`. `.filter()` receives `&Item` = `&&i32` (borrows without consuming) so `&&x` peels both layers. `.map()` receives `Item` = `&i32` so `&x` peels one layer. Both give `x: i32`.

**Score: 3/3**

---

### Q2.3 — Operator ordering bug?

**Thomas's answer:**
No bug spotted. Noted that returning `&str` of string literals is safe because they live in the binary (`&'static str`).

**Feedback:** Good insight on `&'static str` — correct. But didn't address the actual question. The answer is no bug — because longer operators (`>=`, `<=`) are checked before shorter ones (`>`, `<`). If `>` were checked before `>=`, a string like `">=5"` would incorrectly match `>`. Operator ordering was the exact bug category fixed in class 05.

**Score: partial** — right observation, wrong question answered.

---

### Q2.2 — `.position()`

**Thomas's answer:** `Some(1)`

**Feedback:** Correct.

**Score: 3/3**

---

### Q1.10 — `use super::*` in tests

**Thomas's answer:**
`super` loads the parent module and is often used with nested modules, such as in tests that should be contained.

**Feedback:** Correct. Worth adding: `use super::*` also imports private items — child modules can see private items in their parent. This is why it's the idiomatic choice for tests: it lets you test internal functions without making them `pub`. `use crate::...` only reaches public items and is more verbose.

**Score: 2/3** — concept right, private-access detail missing.

---

### Q1.9 — `mod foo;` vs `mod foo { ... }`

**Thomas's answer:**
`mod foo { ... }` contains the module within the braces. They can be nested in other modules.

**Feedback:** Second half correct. First half (why `mod foo;` maps to `src/foo.rs`) not addressed. The answer: it's a compiler convention — `mod foo;` tells the compiler to load the module body from `src/foo.rs` or `src/foo/mod.rs`. Key insight: file system layout does not define the module tree; `mod` declarations do. Two files in the same folder are not in the same scope unless one declares `mod` for the other.

**Score: 1/2**

---

### Q1.8 — `Copy` vs non-`Copy`

**Thomas's answer:**
Copy types (e.g. u32) are small enough to copy without overhead. Non-Copy types like structs and Strings are moved because they're too large to copy automatically.

**Feedback:** Intuition pointing in the right direction but the reason is wrong. It's not about size — it's about heap ownership. `String` is non-`Copy` because it owns heap memory; silently copying it would create two owners of the same allocation (double-free). `u32` is `Copy` because it's entirely stack-allocated with no heap resource. Also: structs are not inherently non-`Copy` — a struct with all-`Copy` fields can `#[derive(Copy, Clone)]`.

**Correct answer:**
- `Copy` = entirely stack-allocated, no heap ownership. Bitwise-duplicated silently. Example: `u32`, `f64`, `bool`.
- Non-`Copy` = owns heap memory. Moved to prevent double-free. Example: `String`, `Vec<T>`.
- Structs can be either, depending on their fields.

**Score: partial** — examples correct, reasoning wrong.

**Follow-up (C++ analogy):** Non-`Copy` types aren't accessed via a pointer from the outside — they *contain* an internal pointer to heap memory (e.g. `String` is `{ ptr, len, capacity }` on the stack). Bitwise-copying it duplicates the pointer, not the heap data → two owners → double-free. C++ analogy: `Copy` ≈ trivially copyable/POD; non-`Copy` ≈ types with non-trivial destructors (`std::string`, `std::vector`, `std::unique_ptr`). Rust enforces the rule at compile time; C++ makes you reason about it manually.

---

