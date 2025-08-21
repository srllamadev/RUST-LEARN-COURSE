完美 🚀  
让我们逐步部署一个 **Rust 版的 Hello World**。我会先展示如何在**本地运行**，然后如何**部署到 Web**（使用 WebAssembly）或作为**二进制文件**。

---

## 1. 安装 Rust

在终端中执行：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

这将安装 `rustc`（编译器）和 `cargo`（项目管理器）。

验证安装：

```bash
rustc --version
cargo --version
```

---

## 2. 创建 Hello World 项目

```bash
cargo new a_01_hello_world
cd a_01_hello_world
```

这将创建以下结构：

```
a_01_hello_world/
 ├─ Cargo.toml
 └─ src/main.rs
```

在 `src/main.rs` 中已经包含：

```rust
fn main() {
    println!("Hello, world!");
}
```

---

## 3. 本地运行

```bash
cargo run
```

输出：

```
Hello, world!
```

---

## 4. 部署为二进制文件

如果你想将其作为可执行文件分享：

```bash
cargo build --release
```

这将在 `target/release/a_01_hello_world`（Windows 上是 `hello_world.exe`）生成二进制文件。运行：

```bash
# Linux/macOS
./target/release/a_01_hello_world

# Windows
.\target\release\a_01_hello_world.exe
```

---

## 5. 在 Web 上部署（使用 WebAssembly）

要在浏览器中运行你的 "Hello World"：

1. 安装 WebAssembly 目标：

   ```bash
   rustup target add wasm32-unknown-unknown
   ```

2. 编译为 WebAssembly：

   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

3. 这将在 `target/wasm32-unknown-unknown/release/` 生成 `hello_world.wasm`。

4. 使用 [wasm-pack](https://rustwasm.github.io/wasm-pack/) 打包用于 Web：

   ```bash
   cargo install wasm-pack
   wasm-pack build --target web
   ```

5. 创建一个 `index.html` 来加载 `.wasm` 文件。示例：

```html
<!DOCTYPE html>
<html>
  <body>
    <script type="module">
      import init from './pkg/hello_world.js';
      init().then(() => {
        console.log('WASM 加载成功 🚀');
      });
    </script>
  </body>
</html>
```

- 使用本地服务器（如 `python3 -m http.server` 或 `npm serve`）提供文件服务。
