# Rust — Data Types (Lesson)

This small crate demonstrates the main Rust data types and includes brief examples.

Contents

- Scalar types: integers, floats, booleans, characters
- Compound types: tuples, arrays, slices
- Ownership & references: simple demonstration with String

Files

- `src/main.rs` — runnable examples showing type annotations and usage.

Quick notes

- Rust is statically typed. Types are inferred when possible, but explicit annotations are common in learning examples.
- Integers: signed (i8, i16, i32, i64, i128, isize) and unsigned (u8, u16, u32, u64, u128, usize).
- Floating point: f32 and f64 (f64 is the default).
- Boolean: `bool` with values `true` or `false`.
- Character: `char`, 4 bytes, Unicode scalar value (e.g., `'a'`, `'λ'`, `'🎉'`).
- Tuple: fixed-size collection with possibly mixed types, accessed by destructuring or index (`tup.0`).
- Array: fixed-size collection with elements of the same type (`[T; N]`).
- Slice: view into contiguous sequence like arrays (`&[T]`).

Try it

Run the example from the project root:

```sh
cd a_02_data_types
cargo run
```

Expected output (values may vary):

- Prints scalar values
- Prints tuple and array contents
- Demonstrates ownership move and borrow

Further exercises

- Experiment with different integer sizes and arithmetic overflow in debug vs release builds.
- Replace the `String` examples with custom struct types and implement methods.
