# Class 19 — CHIP-8 Opcodes, Timers, Renderer, Input, WebAssembly

## Topics Covered

- Implementing remaining CHIP-8 opcodes
- Font sprites in RAM
- Timer thread with `Arc<Mutex<T>>`
- `Box<dyn Trait>` dynamic dispatch in practice
- `MinifbRenderer` with 10× scaling and keyboard input
- Overflow-safe arithmetic
- WSLg for GUI apps in WSL2
- WebAssembly with `wasm-pack` and `wasm-bindgen`
- Cross-compiling to both native Linux and Wasm from the same codebase

---

## Remaining Opcodes

### Font sprites — `FX29`

CHIP-8 stores built-in digit sprites (0–F) in RAM at `0x000–0x04F`. Each digit is 5 bytes tall, 4 pixels wide (upper 4 bits of each byte used). 16 digits × 5 bytes = 80 bytes total.

Font is preloaded in `Memory::new()`:

```rust
memory.ram[0x000..FONT.len()].copy_from_slice(&FONT);
```

`FX29` sets `I` to the sprite address for digit `Vx`:

```rust
(0xF, x, 0x2, 0x9) => self.i = (self.v[x as usize] * 5) as u16,
```

### BCD decode — `FX33`

Splits `Vx` (0–255) into hundreds, tens, ones digits and writes them to `memory[I]`, `memory[I+1]`, `memory[I+2]`:

```rust
(0xF, x, 0x3, 0x3) => {
    let num = self.v[x as usize];
    let ones = num % 10;
    let tens = (num % 100 - ones) / 10;
    let hundreds = (num - tens * 10 - ones) / 100;
    memory.ram[self.i as usize] = hundreds;
    memory.ram[self.i as usize + 1] = tens;
    memory.ram[self.i as usize + 2] = ones;
}
```

**Bug to watch for:** computing `tens` and `hundreds` as unscaled values and then not dividing gives wrong results. Walk through with `num = 19` to verify.

### Input opcodes

`execute()` takes `keystate: &[u8]` — a slice of currently-pressed CHIP-8 key codes.

```rust
// SKP Vx — skip if key Vx pressed
(0xE, x, 0x9, 0xE) => {
    if keystate.iter().any(|k| *k == self.v[x as usize]) {
        self.pc += 2;
    }
}

// SKNP Vx — skip if key Vx NOT pressed
(0xE, x, 0xA, 0x1) => {
    if !keystate.iter().any(|k| *k == self.v[x as usize]) {
        self.pc += 2;
    }
}

// LD Vx, K — wait for keypress (stall by undoing PC increment)
(0xF, x, 0x0, 0xA) => {
    if let Some(k) = keystate.first() {
        self.v[x as usize] = *k;
    } else {
        self.pc -= 2;
    }
}
```

**Stall trick:** `fetch()` already advanced PC by 2. When no key is pressed, `self.pc -= 2` re-executes this instruction next cycle until a key is pressed.

---

## Timers

### Design decision

Timers count down at 60Hz independently of the CPU (~500Hz). They need a separate thread. Since both the CPU (for `FX07`/`FX15`/`FX18` opcodes) and the timer thread access them, they're shared mutable state → `Arc<Mutex<Timers>>`.

Timers live as a field on `Cpu`:

```rust
pub struct Cpu {
    // ...
    pub timers: Arc<Mutex<Timers>>,
}
```

### Timer thread

```rust
pub fn start(timers: Arc<Mutex<Timers>>) {
    std::thread::spawn(move || {
        loop {
            timers.lock().unwrap().tick();
            std::thread::sleep(Duration::from_secs_f64(1.0 / 60.0));
        }
    });
}
```

**NLL in action:** `timers.lock().unwrap().tick()` — the `MutexGuard` is a temporary not bound to a variable, so the compiler drops it at the end of the statement (not the end of the loop). The mutex is released *before* the sleep. If you bound it to `let mut t = ...`, the lock would be held across the sleep — bad.

### Starting the timer thread in main

```rust
Timers::start(cpu.timers.clone());
```

`Arc::clone()` is cheap — just increments a reference count. Both `cpu` and the timer thread hold an `Arc` pointing to the same `Mutex<Timers>` on the heap. `Arc` is Rust's `std::shared_ptr<T>` — "Atomically Reference Counted". The regular `Rc<T>` is the same but not thread-safe.

---

## `Box<dyn Renderer>` — dynamic dispatch in practice

The `Renderer` trait:

```rust
pub trait Renderer {
    fn draw(&mut self, display: &[[bool; 64]; 32]) -> Result<()>;
    fn is_running(&self) -> bool;
    fn pressed_keys(&self) -> Vec<u8>;
}
```

`run()` accepts any renderer — it doesn't know or care about the concrete type:

```rust
fn run(rom: Vec<u8>, mut renderer: Box<dyn Renderer>) -> Result<()> { ... }
```

`main()` decides the concrete type:

```rust
run(rom, Box::new(MinifbRenderer::new()?))
```

To use a `NullRenderer` for headless tests, just pass `Box::new(NullRenderer::new())` instead. The emulator core is unchanged.

---

## MinifbRenderer

### 10× scaling

CHIP-8's 64×32 display is tiny — scale each pixel 10× both horizontally and vertically for a 640×320 window.

```rust
self.buffer = display
    .iter()
    .flat_map(|row| {
        let scaled_row: Vec<u32> = row
            .iter()
            .flat_map(|&p| repeat_n(if p { 0x00FFFFFF } else { 0x00000000 }, 10))
            .collect();
        repeat_n(scaled_row, 10)
    })
    .flatten()
    .collect();
```

Key insight: the row must be collected into a `Vec` before repeating, because `repeat_n` needs ownership and `Clone`.

### Keyboard mapping

CHIP-8's 16-key hex pad maps to keyboard keys:

```
CHIP-8:  1 2 3 C    Keyboard:  1 2 3 4
         4 5 6 D               Q W E R
         7 8 9 E               A S D F
         A 0 B F               Z X C V
```

Using array position as the key value — no big `match` needed:

```rust
let key_dict: [Key; 16] = [Key::X, Key::Key1, Key::Key2, ...];
self.window
    .get_keys()
    .iter()
    .filter_map(|k| key_dict.iter().position(|k2| k2 == k).map(|v| v as u8))
    .collect()
```

---

## Overflow-safe arithmetic

Rust panics on integer overflow in debug mode. CHIP-8 arithmetic should wrap. Use `wrapping_add`/`overflowing_add`:

- `wrapping_add` — wraps silently, returns just the value
- `overflowing_add` — returns `(value, did_overflow)` — use when you need the carry flag (VF)

```rust
// ADD Vx, byte — no carry flag, just wrap
self.v[x as usize] = self.v[x as usize].overflowing_add((opcode & 0xFF) as u8).0;

// ADD Vx, Vy — sets VF to carry
let (result, overflow) = self.v[x as usize].overflowing_add(self.v[y as usize]);
self.v[x as usize] = result;
self.v[0xF] = overflow as u8;
```

Same applies to `SUB` and `SUBN` — use `overflowing_sub`.

---

## WSLg

WSL2 on Windows 11 supports GUI apps via WSLg. Check with `echo $DISPLAY` — if it returns `:0`, you have it. minifb works but may print:

```
Failed to create server-side surface decoration: Missing
```

This is a Wayland compositor warning (missing decoration protocol). Fix by forcing X11:

```
WAYLAND_DISPLAY="" cargo run -- path/to/rom
```

---

---

## WebAssembly

### Architecture change

In native mode, Rust drives the loop. In Wasm, **JS drives the loop** via `requestAnimationFrame` — Rust exposes a struct with methods JS calls each frame.

The `Renderer` trait doesn't apply in Wasm — JS renders to `<canvas>` directly. Instead, expose a `Chip8` struct via `#[wasm_bindgen]`:

```rust
#[wasm_bindgen]
impl Chip8 {
    pub fn new(rom: &[u8]) -> Self { ... }
    pub fn step(&mut self, keys: &[u8]) { ... }
    pub fn tick_timers(&mut self) { ... }
    pub fn get_display(&self) -> Vec<u8> { ... }  // flat array, 0 or 255 per pixel
}
```

Wasm doesn't support threads — remove `Timers::start()` from `new()`. JS calls `tick_timers()` at 60Hz instead.

### Cargo.toml setup

```toml
[lib]
crate-type = ["cdylib", "rlib"]
# cdylib → wasm-pack produces the .wasm file
# rlib   → main.rs can link against the library

[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
minifb = "0.27"

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
getrandom = { version = "0.4", features = ["wasm_js"] }
```

Target-specific dependencies only get included when compiling for that target.

### Conditional module compilation

```rust
// lib.rs
#[cfg(not(target_arch = "wasm32"))]
pub mod renderer;

#[cfg(target_arch = "wasm32")]
pub mod wasm;
```

### Building

```
cargo build                      # native Linux binary
wasm-pack build --target web     # Wasm + JS glue in pkg/
```

### JS game loop

```js
import init, { Chip8 } from '../pkg/chip8.js';
await init();

const chip8 = Chip8.new(romBytes);
let lastTimer = 0;

function loop(ts) {
    const keys = new Uint8Array(pressed);
    for (let i = 0; i < 10; i++) chip8.step(keys);  // ~600Hz at 60fps
    chip8.tick_timers();

    const pixels = chip8.get_display();  // Uint8Array from Vec<u8>
    const img = ctx.createImageData(64, 32);
    for (let i = 0; i < 2048; i++) {
        img.data[i*4] = img.data[i*4+1] = img.data[i*4+2] = pixels[i];
        img.data[i*4+3] = 255;
    }
    ctx.putImageData(img, 0, 0);
    requestAnimationFrame(loop);
}
requestAnimationFrame(loop);
```

Canvas is 64×32 with CSS scaling: `style="width:640px;height:320px;image-rendering:pixelated"`.

Needs a local HTTP server (can't load Wasm from `file://`):
```
python3 -m http.server 8080
```

### `#[wasm_bindgen]` notes

- Put `#[wasm_bindgen]` on both the struct and the `impl` block
- Struct fields must be private (or JS-compatible types) — wasm_bindgen can't expose `Cpu`, `Memory`, `Display` directly
- Methods taking/returning `&[u8]` and `Vec<u8>` work out of the box — wasm_bindgen generates `Uint8Array` bindings automatically
- `wasm_bindgen::prelude::*` brings in the attribute macro

---

## Current State

CHIP-8 emulator fully working on both native Linux (minifb window) and WebAssembly (browser canvas). Same core codebase, same opcodes — only different entry points and renderer implementations. Tested against demo ROMs on both platforms.

**Next steps:**
- `NullRenderer` for headless opcode unit tests
- SDL2 renderer (FFI with C library)
