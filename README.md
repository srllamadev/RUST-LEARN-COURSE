Let's deploy a **Hello World in Rust**, step by step. I'll first show you how to run it **locally**, then how to **deploy it on the web** (using WebAssembly) or as a **binary**.

---

## 1. Install Rust

In your terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This installs `rustc` (the compiler) and `cargo` (the project manager).

Verify:

```bash
rustc --version
cargo --version
```

---

## 2. Create a Hello World Project

```bash
cargo new a_01_hello_world
cd a_01_hello_world
```

This creates the structure:

```
a_01_hello_world/
 ├─ Cargo.toml
 └─ src/main.rs
```

In `src/main.rs`, you’ll already have:

```rust
fn main() {
    println!("Hello, world!");
}
```

---

## 3. Run Locally

```bash
cargo run
```

Output:

```
Hello, world!
```

---

## 4. Deploy as a Binary

To share it as an executable:

```bash
cargo build --release
```

The binary will be generated at `target/release/hello_world` (or `hello_world.exe` on Windows). Run it with:

```bash
# Linux/macOS
./target/release/hello_world

# Windows
.\target\release\a_01_hello_world.exe
```

---

## 5. Deploy on the Web (Optional with WebAssembly)

To run your "Hello World" in a browser:

1. Install the WebAssembly target:

   ```bash
   rustup target add wasm32-unknown-unknown
   ```

2. Compile to WebAssembly:

   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

3. This generates `hello_world.wasm` in `target/wasm32-unknown-unknown/release/`.

4. Use [wasm-pack](https://rustwasm.github.io/wasm-pack/) to bundle it for the web:

   ```bash
   cargo install wasm-pack
   wasm-pack build --target web
   ```

5. Create an `index.html` to load the `.wasm` file. Example:

```html
<!DOCTYPE html>
<html>
  <body>
    <script type="module">
      import init from './pkg/hello_world.js';
      init().then(() => {
        console.log('WASM loaded 🚀');
      });
    </script>
  </body>
</html>
```

- Serve the files with a local server (e.g., `python3 -m http.server` or `npm serve`).
