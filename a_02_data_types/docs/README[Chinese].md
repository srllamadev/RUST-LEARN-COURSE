# 如何执行

进入项目目录并运行：

```bash
cargo run
```

## 可用语言

本文档也提供其他语言版本：

- [Chinese](docs/README[Chinese].md)
- [Hindi](docs/README[French].md)
- [Spanish](docs/README[German].md)
- [Spanish](docs/README[Hindi].md)
- [Spanish](docs/README[Portuguese].md)
- [Spanish](docs/README[Russian].md)
- [Spanish](docs/README[Spanish].md)
- [English](../README.md)

## Rust 数据类型和所有权

本课程介绍 Rust 的数据类型以及所有权和借用的基本概念。理解这些概念对于编写安全高效的 Rust 代码至关重要。

## 目录

- [🦙 标量类型](#-标量类型)
- [🦙 复合类型](#-复合类型)
- [🦙 所有权](#-所有权)
- [🦙 引用和借用](#-引用和借用)
- [🚀 运行示例](#-运行示例)
- [🦙 关键概念](#-关键概念)
- [🦙 练习](#-练习)

## 🦙 标量类型

标量类型表示单个值。Rust 有四种主要的标量类型：整数、浮点数、布尔值和字符。

### 整数

Rust 提供了具有不同大小和有符号/无符号变体的各种整数类型：

```rust
let x: i32 = -42;    // 有符号 32 位整数
let y: u64 = 42;     // 无符号 64 位整数
```

### 浮点数

Rust 支持两种浮点类型：

```rust
let f: f64 = 3.1415; // 64 位浮点数
```

### 布尔值

布尔值表示真或假：

```rust
let b: bool = true;  // 布尔值
```

### 字符

字符表示单个 Unicode 标量值：

```rust
let c: char = 'λ';   // Unicode 字符（lambda）
```

## 📦 复合类型

复合类型将多个值分组为一个类型。

### 元组

元组是固定大小的集合，可以包含混合类型：

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);  // 元组声明
let (a, b, c) = tup;                       // 解构
println!("tuple: ({}, {}, {})", a, b, c);  // 访问元素
```

### 数组

数组是相同类型元素的固定大小集合：

```rust
let arr: [i32; 4] = [1, 2, 3, 4];  // 数组声明
println!("array: {:?}", arr);       // 打印整个数组
```

### 切片

切片提供对数组的视图而不获取所有权：

```rust
fn print_slice(slice: &[i32]) {
    println!("slice: {:?}", slice);
}
print_slice(&arr);  // 将数组作为切片传递
```

## 🦙 所有权

所有权是 Rust 最独特的特性，能够在没有垃圾收集器的情况下实现内存安全。

### 所有权规则

1. **每个值都有一个变量作为其所有者**
2. **每个值在任何时候只能有一个所有者**
3. **当所有者超出作用域时，该值将被丢弃**

```rust
let s = String::from("hello");  // s 拥有 String
takes_ownership(s);             // s 被移动到函数中
// s 在这里不再有效
```

### 移动 vs 复制

- **移动**：转移所有权（对于堆分配数据，如 `String`）
- **复制**：创建副本（对于仅栈数据，如整数）

```rust
let x = 5;        // i32 实现了 Copy
let y = x;        // x 被复制，两者都有效
println!("x: {}, y: {}", x, y);  // 正常工作

let s1 = String::from("hello");  // String 没有实现 Copy
let s2 = s1;                     // s1 被移动到 s2
// println!("{}", s1);           // 这不会编译
```

## 🦙 引用和借用

引用允许您在不获取所有权的情况下访问数据。

### 不可变引用

```rust
let s2 = String::from("world");
borrow_string(&s2);              // 不可变借用 s2
println!("s2 after borrow: {}", s2);  // s2 仍然有效
```

### 可变引用

```rust
fn change_string(s: &mut String) {
    s.push_str(" world");
}

let mut s = String::from("hello");
change_string(&mut s);
println!("{}", s);  // 打印 "hello world"
```

### 引用规则

1. **您可以拥有一个可变引用或多个不可变引用**
2. **引用必须始终有效**

## 🚀 运行示例

### 先决条件

确保您的系统上已安装 Rust。

```bash
# 检查 Rust 是否已安装
rustc --version
cargo --version
```

### 编译和执行

```bash
# 导航到项目目录
cd a_02_data_types

# 使用 Cargo 运行（推荐）
cargo run

# 或手动编译
cargo build
./target/debug/a_02_data_types.exe  # 在 Windows 上
# 或
./target/debug/a_02_data_types      # 在 Linux/macOS 上
```

### 预期输出

```text
i32: -42 | u64: 42 | f64: 3.1415 | bool: true | char: λ
tuple: (500, 6.4, 1)
array: [1, 2, 3, 4]
slice: [1, 2, 3, 4]
I took ownership of: hello
I borrowed: world
s2 after borrow: world
```

## 🦙 关键概念

### 内存管理

- **栈**：快速、自动分配/释放、固定大小
- **堆**：较慢、手动分配、可变大小
- **所有权**：确保内存安全并防止内存泄漏

### 借用的好处

- **零成本抽象**：引用不会增加运行时开销
- **防止数据竞争**：可变借用规则防止并发修改
- **灵活**：可以不可变借用多次或可变借用一次

### 常见模式

1. **当您不需要所有权时使用引用**
2. **当您需要修改借用数据时使用 `&mut`**
3. **当调用者应该获取所有权时返回拥有的值**
4. **使用切片进行灵活的数组访问**

### 性能影响

- **复制类型**（整数、浮点数、布尔值、字符）：传递成本低廉
- **移动类型**（String、Vec）：尽可能转移所有权
- **借用类型**：使用引用避免不必要的复制

## 🌚 练习

1. **类型探索**：尝试不同的标量类型及其范围
2. **元组操作**：创建具有不同类型组合的元组并练习解构
3. **数组操作**：创建不同大小的数组并使用索引访问元素
4. **所有权转移**：编写获取所有权 vs. 借用参数的函数
5. **引用练习**：创建接受可变和不可变引用的函数
6. **切片操作**：练习从数组创建切片并将其传递给函数

## 进一步阅读

- [Rust 书籍 - 数据类型](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Rust 书籍 - 所有权](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Rust 书籍 - 引用和借用](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Rust 参考 - 类型](https://doc.rust-lang.org/reference/types.html)

## 笔记

- **所有权** 是 Rust 实现内存安全的秘密武器
- **借用** 允许高效代码而不牺牲安全性
- **类型** 决定值是被复制还是移动
- 定期练习这些概念 - 它们是 Rust 编程的基础
