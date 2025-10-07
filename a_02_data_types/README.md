## How to execute

Go to the project directory and run:

```bash
cargo run
```


## Available Languages

This documentation is also available in other languages:

- [Chinese](docs/README[Chinese].md)
- [Hindi](docs/README[French].md)
- [Spanish](docs/README[German].md)
- [Spanish](docs/README[Hindi].md)
- [Spanish](docs/README[Portuguese].md)
- [Spanish](docs/README[Russian].md)
- [Spanish](docs/README[Spanish].md)

# Rust Data Types and Ownership

This lesson introduces Rust's data types and fundamental concepts of ownership and borrowing. Understanding these concepts is crucial for writing safe and efficient Rust code.

## Table of Contents

- [🦙 Scalar Types](#-scalar-types)
- [🦙 Compound Types](#-compound-types)
- [🦙 Ownership](#-ownership)
- [🦙 References and Borrowing](#-references-and-borrowing)
- [🚀 Running the Example](#-running-the-example)
- [🦙 Key Concepts](#-key-concepts)
- [🦙 Exercises](#-exercises)

## 🦙 Scalar Types

Scalar types represent single values. Rust has four primary scalar types: integers, floating-point numbers, booleans, and characters.

### Integers

Rust provides various integer types with different sizes and signed/unsigned variants:

```rust
let x: i32 = -42;    // Signed 32-bit integer
let y: u64 = 42;     // Unsigned 64-bit integer
```

### Floating-Point Numbers

Rust supports two floating-point types:

```rust
let f: f64 = 3.1415; // 64-bit floating-point number
```

### Booleans

Boolean values represent true or false:

```rust
let b: bool = true;  // Boolean value
```

### Characters

Characters represent single Unicode scalar values:

```rust
let c: char = 'λ';   // Unicode character (lambda)
```

## 📦 Compound Types

Compound types group multiple values into one type.

### Tuples

Tuples are fixed-size collections that can contain mixed types:

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);  // Tuple declaration
let (a, b, c) = tup;                       // Destructuring
println!("tuple: ({}, {}, {})", a, b, c);  // Accessing elements
```

### Arrays

Arrays are fixed-size collections of elements of the same type:

```rust
let arr: [i32; 4] = [1, 2, 3, 4];  // Array declaration
println!("array: {:?}", arr);       // Print entire array
```

### Slices

Slices provide a view into arrays without taking ownership:

```rust
fn print_slice(slice: &[i32]) {
    println!("slice: {:?}", slice);
}
print_slice(&arr);  // Pass array as slice
```

## 🦙 Ownership

Ownership is Rust's most unique feature and enables memory safety without a garbage collector.

### Ownership Rules

1. **Each value has a variable that's its owner**
2. **Each value can have only one owner at a time**
3. **When the owner goes out of scope, the value is dropped**

```rust
let s = String::from("hello");  // s owns the String
takes_ownership(s);             // s is moved to the function
// s is no longer valid here
```

### Moving vs Copying

- **Move**: Transfers ownership (for heap-allocated data like `String`)
- **Copy**: Creates a duplicate (for stack-only data like integers)

```rust
let x = 5;        // i32 implements Copy
let y = x;        // x is copied, both are valid
println!("x: {}, y: {}", x, y);  // Works fine

let s1 = String::from("hello");  // String does not implement Copy
let s2 = s1;                     // s1 is moved to s2
// println!("{}", s1);           // This would not compile
```

## 🦙 References and Borrowing

References allow you to access data without taking ownership.

### Immutable References

```rust
let s2 = String::from("world");
borrow_string(&s2);              // Borrow s2 immutably
println!("s2 after borrow: {}", s2);  // s2 is still valid
```

### Mutable References

```rust
fn change_string(s: &mut String) {
    s.push_str(" world");
}

let mut s = String::from("hello");
change_string(&mut s);
println!("{}", s);  // Prints "hello world"
```

### Reference Rules

1. **You can have either one mutable reference OR multiple immutable references**
2. **References must always be valid**

## 🚀 Running the Example

### Prerequisites

Make sure you have Rust installed on your system.

```bash
# Check if Rust is installed
rustc --version
cargo --version
```

### Compilation and Execution

```bash
# Navigate to the project directory
cd a_02_data_types

# Run with Cargo (recommended)
cargo run

# Or compile manually
cargo build
./target/debug/a_02_data_types.exe  # On Windows
# or
./target/debug/a_02_data_types      # On Linux/macOS
```

### Expected Output

```text
i32: -42 | u64: 42 | f64: 3.1415 | bool: true | char: λ
tuple: (500, 6.4, 1)
array: [1, 2, 3, 4]
slice: [1, 2, 3, 4]
I took ownership of: hello
I borrowed: world
s2 after borrow: world
```

## 🦙 Key Concepts

### Memory Management

- **Stack**: Fast, automatic allocation/deallocation, fixed size
- **Heap**: Slower, manual allocation, variable size
- **Ownership**: Ensures memory safety and prevents memory leaks

### Borrowing Benefits

- **Zero-cost abstractions**: References don't add runtime overhead
- **Prevents data races**: Mutable borrowing rules prevent concurrent modification
- **Flexible**: Can borrow immutably multiple times or mutably once

### Common Patterns

1. **Use references when you don't need ownership**
2. **Use `&mut` when you need to modify borrowed data**
3. **Return owned values when the caller should take ownership**
4. **Use slices for flexible array access**

### Performance Implications

- **Copy types** (integers, floats, booleans, characters): Cheap to pass around
- **Move types** (String, Vec): Transfer ownership when possible
- **Borrow types**: Use references to avoid unnecessary copying

## 🌚 Exercises

1. **Type Exploration**: Experiment with different scalar types and their ranges
2. **Tuple Operations**: Create tuples with different combinations of types and practice destructuring
3. **Array Manipulation**: Create arrays of different sizes and access elements using indexing
4. **Ownership Transfer**: Write functions that take ownership vs. borrow parameters
5. **Reference Practice**: Create functions that accept mutable and immutable references
6. **Slice Operations**: Practice creating slices from arrays and passing them to functions

## Further Reading

- [Rust Book - Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Rust Book - Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Rust Book - References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Rust Reference - Types](https://doc.rust-lang.org/reference/types.html)

## Notes

- **Ownership** is Rust's secret weapon for memory safety
- **Borrowing** allows efficient code without sacrificing safety
- **Types** determine whether values are copied or moved
- Practice these concepts regularly - they're fundamental to Rust programming

