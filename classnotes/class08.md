# Class 08 — HashMap, group-by, borrow lifetimes

## Topics Covered

- `HashMap<K, V>` — insertion, lookup, the `entry` API
- Sorting a `HashMap` by collecting to `Vec`
- `BTreeMap` as a sorted-map alternative
- How immutable borrows lock mutation (and when the lock lifts)
- Non-Lexical Lifetimes (NLL)

---

## HashMap basics

```rust
use std::collections::HashMap;

let mut counts: HashMap<String, u32> = HashMap::new();
```

`HashMap<K, V>` — key type `K`, value type `V`. O(1) average insert/lookup. No guaranteed iteration order.

### The `entry` API — idiomatic "insert if absent, then increment"

```rust
let entry = counts.entry(field).or_insert(0);
*entry += 1;
```

- **`.entry(key)`** — looks up the key. Returns an `Entry` enum: either the existing slot or a vacant one.
- **`.or_insert(default)`** — if the key didn't exist, inserts `default` and returns `&mut V`. If it did exist, returns `&mut V` to the existing value. Either way you get a mutable reference.
- **`*entry += 1`** — dereference the `&mut u32` to mutate the value behind the pointer.

This does the lookup exactly once — more efficient than a separate `contains_key` + `insert`.

---

## Sorting a HashMap

`HashMap` has no inherent order. To print sorted, collect entries into a `Vec` and sort that:

```rust
let mut sorted: Vec<(&String, &u32)> = counts.iter().collect();
sorted.sort_by_key(|(entry, _)| entry.as_str());
for (entry, count) in sorted {
    println!("{entry:<20} {count:>3}");
}
```

- **`counts.iter()`** — yields `(&K, &V)` pairs (borrowed references into the map).
- **`sort_by_key`** — like `sort_by` but takes a key-extraction closure instead of a comparator. Cleaner when sorting on a single field.
- **`(&String, &u32)`** — the Vec holds references into `counts`, not copies. No cloning needed.

---

## BTreeMap — the sorted alternative

```rust
use std::collections::BTreeMap;

let mut counts: BTreeMap<String, u32> = BTreeMap::new();
// insert same as HashMap
for (entry, count) in &counts {  // already sorted by key
    println!("{entry:<20} {count:>3}");
}
```

`BTreeMap` keeps keys in sorted order automatically (backed by a B-tree).

| | `HashMap` | `BTreeMap` |
|---|---|---|
| Insert/lookup | O(1) average | O(log n) |
| Iteration order | random | sorted by key |
| Use when | performance matters | you need sorted output |

---

## How borrows lock mutation

When `sorted` holds `&String` / `&u32` references into `counts`, the borrow checker sees `counts` as **immutably borrowed** for as long as those references are alive. Mutation is blocked during that window:

```rust
let sorted: Vec<(&String, &u32)> = counts.iter().collect(); // borrow starts
counts.insert("New York".to_string(), 99); // ERROR: cannot borrow as mutable
                                            // immutable borrow still active
```

The rule: you can have **one `&mut`** OR **any number of `&`** — never both at the same time. Holding shared references into a collection locks out all mutation. This prevents the class of bugs (dangling pointers, stale views) that are common in C++ and Java.

---

## Non-Lexical Lifetimes (NLL)

The borrow ends at the **last use**, not at the closing brace. The compiler tracks this:

```rust
let sorted: Vec<(&String, &u32)> = counts.iter().collect();
for (entry, count) in &sorted {
    println!("{entry} {count}");
} // last use of sorted — borrow of counts ends HERE

counts.insert("Seattle".to_string(), 1); // fine — sorted no longer used
// (even though `sorted` hasn't gone out of scope yet)
```

This feature is called **Non-Lexical Lifetimes**. Before NLL, borrows lasted until the closing brace of the scope, which was unnecessarily restrictive. NLL makes the compiler smarter: it ends borrows as early as possible.

---

## Feature added: `--group-by <field>`

```
csvtool data/people.csv --group-by city
```

Implementation:
- Added `groupby: Option<String>` to `Config`
- Parsed `--group-by <field>` in `Config::new()` (same pattern as `--sort`, `--limit`)
- In `run()`: build `HashMap<String, u32>` by iterating `&people`, matching on the field name, then collect + sort + print

The field dispatch uses the same `match groupby.as_str()` pattern used throughout — consistent with the rest of the codebase.

---

## Key takeaways

- `entry().or_insert(default)` is the idiomatic HashMap "upsert" — one lookup, no redundancy.
- `HashMap` is unordered — to sort output, collect to `Vec` and sort.
- `BTreeMap` trades O(1) for automatic key ordering — pick based on need.
- Holding a `&` reference into a collection prevents mutation for the duration of that borrow — by design, not accident.
- NLL means borrows end at last use, not at end of scope — the compiler is smarter than it looks.
