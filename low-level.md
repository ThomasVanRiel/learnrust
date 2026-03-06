# From Rust Source to Machine Code

A practical guide to the compilation pipeline — what happens between `cargo build` and a running binary.

---

## The pipeline

```
your .rs file
     │
     ▼
  Lexing          → tokens (keywords, identifiers, literals, punctuation)
     │
     ▼
  Parsing         → AST (Abstract Syntax Tree)
     │
     ▼
  HIR             → High-level IR: desugared AST, type checking, borrow checking
     │
     ▼
  MIR             → Mid-level IR: control flow graph, further borrow checking, optimizations
     │
     ▼
  LLVM IR         → handed to LLVM (the same backend used by Clang/C++)
     │
     ▼
  Assembly (.s)   → human-readable machine instructions
     │
     ▼
  Object code     → assembled binary (`.o`)
     │
     ▼
  Linked binary   → final executable, with stdlib and dependencies linked in
```

Rust uses **LLVM** as its backend. This means Rust gets decades of battle-tested optimizations for free — the same ones used by C, C++, and Swift. LLVM knows how to vectorize loops, inline functions, eliminate dead code, and much more.

---

## The stages in detail

### Lexing

The raw source text is broken into **tokens** — the smallest meaningful units:

```
fn add(a: i32, b: i32) -> i32 { a + b }
```

Becomes: `fn`, `add`, `(`, `a`, `:`, `i32`, `,`, `b`, `:`, `i32`, `)`, `->`, `i32`, `{`, `a`, `+`, `b`, `}`

No meaning yet — just recognising shapes.

### Parsing → AST

Tokens are arranged into a tree representing the grammatical structure of the program. The AST for `a + b` is roughly:

```
BinaryOp(+)
├── Ident(a)
└── Ident(b)
```

Macros are expanded here. `println!("hi")` becomes the actual function calls it expands to.

### HIR (High-level Intermediate Representation)

The AST is **desugared** — syntactic sugar is removed and translated into simpler forms. For loops become `loop` + `match`. `?` becomes `match result { Ok(v) => v, Err(e) => return Err(e) }`.

**Type checking and the borrow checker run here.** This is where lifetime annotations are verified and then discarded — they don't exist past this stage.

### MIR (Mid-level Intermediate Representation)

A much simpler representation: a **control flow graph** (CFG) where code is broken into basic blocks with explicit jumps between them. No more nested expressions — everything is flattened into simple assignments and gotos.

MIR is Rust-specific and quite readable. It's where the borrow checker does its final checks and where Rust performs many optimizations before handing off to LLVM.

### LLVM IR

A low-level, typed, assembly-like language. This is what LLVM receives. From here, LLVM applies its own extensive optimization passes (inlining, dead code elimination, vectorization, etc.) and generates assembly for the target architecture.

### Assembly → Object code → Binary

LLVM emits assembly (`.s`), which is assembled into object code (`.o`), which is then **linked** — the linker combines your object code with the standard library and any dependencies into the final executable.

---

## Hands-on: inspecting each stage

Set up a scratch project with a simple function to study:

```
cargo new lowlevel
cd lowlevel
```

Put this in `src/main.rs`:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn main() {
    println!("{}", add(2, 3));
    println!("{}", max(10, 7));
}
```

Use `pub` on the functions — without it the compiler may inline or eliminate them entirely in release mode (which is good for performance but makes them invisible to inspect).

---

### Stage 1: MIR

```
cargo rustc -- --emit=mir
```

Output lands in `target/debug/deps/lowlevel-<hash>.mir`. Open it and look for your functions. MIR for `add` will look something like:

```
fn add(_1: i32, _2: i32) -> i32 {
    let mut _0: i32;

    bb0: {
        _0 = Add(_1, _2);
        return;
    }
}
```

**Reading MIR:**

- `bb0`, `bb1`, etc. — **basic blocks**: straight-line sequences of code with no branches
- Branches appear as jumps between blocks (`goto bb1` or `switchInt`)
- `_0` is always the return value
- Every expression is flattened — no nesting

Look at `max` in the MIR output — you'll see it split into multiple basic blocks with a conditional jump. That's the if/else translated to a CFG.

---

### Stage 2: LLVM IR

```
cargo rustc -- --emit=llvm-ir
```

Output: `target/debug/deps/lowlevel-<hash>.ll`

LLVM IR for `add`:

```llvm
define i32 @add(i32 %a, i32 %b) {
  %result = add i32 %a, %b
  ret i32 %result
}
```

LLVM IR is typed and SSA form (Static Single Assignment — every variable assigned exactly once). It looks like a typed assembly language. You can see how Rust types (`i32`) map to LLVM types (`i32`).

---

### Stage 3: Assembly

```
cargo rustc --release -- --emit=asm
```

Use `--release` here. Debug mode adds enormous amounts of unwind/debug metadata that obscures the actual logic. Release mode gives clean output.

Output: `target/release/deps/lowlevel-<hash>.s`

The `add` function in x86-64 assembly:

```asm
add:
    lea eax, [rdi + rsi]   ; or just: add edi, esi / mov eax, edi
    ret
```

`rdi` and `rsi` are the first two integer arguments (x86-64 calling convention: arguments go in `rdi`, `rsi`, `rdx`, `rcx`, `r8`, `r9` in order). The return value goes in `rax`/`eax`.

With optimization, LLVM may compile `add(a, b)` to a single `lea` instruction — Load Effective Address, repurposed for arithmetic.

---

## Godbolt — Compiler Explorer

The best tool for this: **<https://godbolt.org>**

- Paste Rust code on the left
- Pick `rustc` as the compiler
- Add flags: `-O` for optimized output (equivalent to `--release`)
- See assembly on the right, colour-coded to match source lines

Tips:

- Add `-C opt-level=3` for maximum optimization (same as `--release`)
- Add `-C opt-level=0` for debug (unoptimized, easier to follow 1:1)
- Hover over assembly lines — godbolt highlights the corresponding source

This is the fastest way to answer "what does this code compile to?" without leaving your browser.

---

## Things to study

### 1. Simple arithmetic — what's the minimum assembly?

```rust
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

With `-O`: often a single instruction. The function call overhead may even disappear if the caller inlines it.

### 2. Branching — if/else

```rust
pub fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}
```

With optimization, LLVM often compiles this to a `cmov` (conditional move) — branchless code. No jump instruction at all. This is a common optimization because branch mispredictions are expensive on modern CPUs.

### 3. Loops

```rust
pub fn sum(n: i32) -> i32 {
    let mut total = 0;
    for i in 0..n {
        total += i;
    }
    total
}
```

With `-O`: LLVM may **vectorize** this — computing multiple iterations simultaneously using SIMD instructions. Or it may recognize the arithmetic series formula and replace the whole loop with a multiply. Compilers are frighteningly good.

### 4. Stack vs heap

```rust
pub fn on_stack() -> i32 {
    let x = 42_i32;
    x
}

pub fn on_heap() -> Box<i32> {
    Box::new(42)
}
```

`on_stack`: no memory operations — the value lives in a register.
`on_heap`: calls `__rust_alloc` (the allocator), stores `42` at the returned address, wraps the pointer in a `Box`. You'll see the allocation call in the assembly.

### 5. References are just pointers

```rust
pub fn double(x: &i32) -> i32 {
    *x * 2
}
```

The `&i32` is passed as a pointer in `rdi`. The function dereferences it (loads from the address) and multiplies. No lifetime information — that was compile-time only.

### 6. Debug vs release

Compile the same function both ways and compare. Debug builds:

- No inlining
- Variables kept in memory (not registers) for debugger access
- Bounds checks on every array/slice access
- Panic infrastructure included

Release builds:

- Aggressive inlining
- Variables in registers
- Bounds checks often eliminated (when the compiler can prove they'll never fire)
- Dead code removed

---

## Key concepts

### Calling convention

x86-64 Linux uses the **System V AMD64 ABI**:

- First 6 integer/pointer args: `rdi`, `rsi`, `rdx`, `rcx`, `r8`, `r9`
- Return value: `rax`
- Caller saves: `rax`, `rcx`, `rdx`, `rsi`, `rdi`, `r8`, `r9`, `r10`, `r11`
- Callee saves: `rbx`, `rbp`, `r12`–`r15`

Windows uses a different convention (first 4 args in `rcx`, `rdx`, `r8`, `r9`).

### Stack frame

Each function call gets a **stack frame** — a region of the stack for its local variables and saved registers. `rsp` (stack pointer) moves down on entry and back up on return. On x86-64 the stack grows downward.

```
high address
┌───────────────┐
│  caller frame │
├───────────────┤ ← rsp before call
│  return addr  │  pushed by `call` instruction
├───────────────┤
│  saved rbp    │  if frame pointer is used
├───────────────┤
│  local vars   │
├───────────────┤ ← rsp during function
│               │
low address
```

### Inlining

When the compiler decides a function is small enough, it copies the function body directly into the call site — eliminating the `call`/`ret` overhead and enabling further optimizations. This is why release builds can be dramatically faster.

### Zero-cost abstractions

Rust's promise: iterators, closures, generics, traits — these compile to the same assembly as hand-written loops and direct function calls. You can verify this on godbolt. A `.iter().map().filter().sum()` chain often compiles identically to a hand-written `for` loop with an accumulator.

---

## Recommended exercises

1. **Try `add` on godbolt** with opt-level 0 vs 3. Watch it go from a full stack frame to one instruction.

2. **Try `max`** — look for `cmov` (conditional move without a branch) in the optimized output.

3. **Try a closure vs a loop:**

```rust
pub fn sum_closure(v: &[i32]) -> i32 {
    v.iter().sum()
}
pub fn sum_loop(v: &[i32]) -> i32 {
    let mut total = 0;
    for x in v { total += x; }
    total
}
```

Compare the assembly. They should be identical (or very close) — zero-cost abstraction in action.

1. **Try `Box::new(42)` vs `let x = 42`** — spot the allocator call in the heap version.

2. **Find a bounds check** — index a slice with a variable index in debug mode and find the panic branch in the assembly.
