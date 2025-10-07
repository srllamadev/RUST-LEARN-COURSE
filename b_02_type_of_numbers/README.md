# Rust Number Types / Tipos de Números en Rust

This lesson demonstrates the different number types available in Rust, including integers and floating-point numbers.

Esta lección demuestra los diferentes tipos de números disponibles en Rust, incluyendo enteros y números de punto flotante.

## 📋 Table of Contents / Tabla de Contenidos

- [Integer Types / Tipos Enteros](#integer-types--tipos-enteros)
- [Floating-Point Types / Tipos de Punto Flotante](#floating-point-types--tipos-de-punto-flotante)
- [Number Literals / Literales Numéricos](#number-literals--literales-numéricos)
- [Type Inference / Inferencia de Tipos](#type-inference--inferencia-de-tipos)
- [Arithmetic Operations / Operaciones Aritméticas](#arithmetic-operations--operaciones-aritméticas)
- [Running the Example / Ejecutar el Ejemplo](#running-the-example--ejecutar-el-ejemplo)

## 🔢 Integer Types / Tipos Enteros

Rust provides several integer types with different sizes and signed/unsigned variants.

Rust proporciona varios tipos enteros con diferentes tamaños y variantes con/sin signo.

### Unsigned Integers / Enteros Sin Signo

Unsigned integers can only represent positive numbers (including zero).

Los enteros sin signo solo pueden representar números positivos (incluyendo cero).

| Type / Tipo | Size / Tamaño | Range / Rango | Example / Ejemplo |
|-------------|---------------|---------------|-------------------|
| `u8` | 8 bits | 0 to 255 | `let x: u8 = 255;` |
| `u16` | 16 bits | 0 to 65,535 | `let x: u16 = 65_535;` |
| `u32` | 32 bits | 0 to 4,294,967,295 | `let x: u32 = 4_294_967_295;` |
| `u64` | 64 bits | 0 to 18,446,744,073,709,551,615 | `let x: u64 = 18_446_744_073_709_551_615;` |
| `u128` | 128 bits | 0 to 340,282,366,920,938,463,463,374,607,431,768,211,455 | `let x: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;` |
| `usize` | Architecture dependent / Dependiente de la arquitectura | 32-bit: 0 to 4,294,967,295 64-bit: 0 to 18,446,744,073,709,551,615 | `let x: usize = 100;` |

### Signed Integers / Enteros Con Signo

Signed integers can represent both positive and negative numbers.

Los enteros con signo pueden representar tanto números positivos como negativos.

| Type / Tipo | Size / Tamaño | Range / Rango | Example / Ejemplo |
|-------------|---------------|---------------|-------------------|
| `i8` | 8 bits | -128 to 127 | `let x: i8 = -128;` |
| `i16` | 16 bits | -32,768 to 32,767 | `let x: i16 = 32_767;` |
| `i32` | 32 bits | -2,147,483,648 to 2,147,483,647 | `let x: i32 = -2_147_483_648;` |
| `i64` | 64 bits | -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807 | `let x: i64 = 9_223_372_036_854_775_807;` |
| `i128` | 128 bits | -170,141,183,460,469,231,731,687,303,715,884,105,728 to 170,141,183,460,469,231,731,687,303,715,884,105,727 | `let x: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728;` |
| `isize` | Architecture dependent / Dependiente de la arquitectura | 32-bit: -2,147,483,648 to 2,147,483,647 64-bit: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807 | `let x: isize = -50;` |

## 📊 Floating-Point Types / Tipos de Punto Flotante

Floating-point types represent decimal numbers with different precision levels.

Los tipos de punto flotante representan números decimales con diferentes niveles de precisión.

| Type / Tipo | Size / Tamaño | Precision / Precisión | Example / Ejemplo |
|-------------|---------------|----------------------|-------------------|
| `f32` | 32 bits | ~6-9 decimal digits / ~6-9 dígitos decimales | `let x: f32 = 3.141592;` |
| `f64` | 64 bits | ~15-17 decimal digits / ~15-17 dígitos decimales | `let x: f64 = 2.718281828459045;` |

**Note / Nota**: `f64` is the default floating-point type in Rust and is generally preferred for its higher precision.

## 🔤 Number Literals / Literales Numéricos

Rust supports various ways to write number literals for better readability.

Rust soporta varias formas de escribir literales numéricos para mejor legibilidad.

### Underscore Separator / Separador de Guion Bajo

```rust
let million = 1_000_000;        // Same as 1000000
let binary = 0b_1111_0000;      // Binary with separators
let hex = 0x_DEAD_BEEF;         // Hexadecimal with separators
```

### Different Bases / Diferentes Bases

```rust
let decimal = 42;               // Decimal (default)
let hex = 0x2A;                 // Hexadecimal (0x prefix)
let octal = 0o52;               // Octal (0o prefix)
let binary = 0b101010;          // Binary (0b prefix)
```

### Type Suffixes / Sufijos de Tipo

```rust
let x = 42u32;                  // u32 suffix
let y = 3.14f32;                // f32 suffix
let z = 1_000_000i64;           // i64 suffix
```

## 🎯 Type Inference / Inferencia de Tipos

Rust can often infer the type of a variable from its context.

Rust puede inferir el tipo de una variable desde su contexto.

```rust
let x = 42;         // i32 (default integer type)
let y = 3.14;       // f64 (default float type)
let z = 42u8;       // u8 (explicit type annotation)
```

## ➗ Arithmetic Operations / Operaciones Aritméticas

All number types support basic arithmetic operations.

Todos los tipos numéricos soportan operaciones aritméticas básicas.

```rust
let a = 10;
let b = 3;

let sum = a + b;        // Addition / Suma
let difference = a - b; // Subtraction / Resta
let product = a * b;    // Multiplication / Multiplicación
let quotient = a / b;   // Division / División
let remainder = a % b;  // Modulo / Módulo
```

### Overflow Behavior / Comportamiento de Desbordamiento

- **Debug mode**: Program panics on integer overflow
- **Release mode**: Wraps around (two's complement)

- **Modo debug**: El programa entra en pánico con desbordamiento de enteros
- **Modo release**: Se envuelve (complemento a dos)

```rust
let x: u8 = 255;
let y = x + 1;  // Panics in debug, wraps to 0 in release
```

## 🚀 Running the Example / Ejecutar el Ejemplo

### Prerequisites / Prerrequisitos

Make sure you have Rust installed on your system.

Asegúrate de tener Rust instalado en tu sistema.

```bash
# Check if Rust is installed / Verificar si Rust está instalado
rustc --version
cargo --version
```

### Compilation and Execution / Compilación y Ejecución

```bash
# Navigate to the project directory / Navegar al directorio del proyecto
cd b_02_type_of_numbers

# Compile the program / Compilar el programa
rustc 02_type_of_numbers.rs

# Run the executable / Ejecutar el ejecutable
./02_type_of_numbers.exe  # On Windows / En Windows
# or
./02_type_of_numbers      # On Linux/macOS / En Linux/macOS
```

### Expected Output / Salida Esperada

```
Unsigned integers: u8=255 u16=65535 u32=4294967295 u64=18446744073709551615 u128=340282366920938463463374607431768211455 usize=100
Signed integers: i8=-128 i16=32767 i32=-2147483648 i64=9223372036854775807 i128=-170141183460469231731687303715884105728 isize=-50
Floating numbers: f32=3.141592 f64=2.718281828459045
This is printed on the same line!
```

## 📚 Key Concepts / Conceptos Clave

### Memory Usage / Uso de Memoria

- `u8` and `i8`: 1 byte
- `u16` and `i16`: 2 bytes
- `u32` and `i32`: 4 bytes
- `u64` and `i64`: 8 bytes
- `u128` and `i128`: 16 bytes
- `f32`: 4 bytes
- `f64`: 8 bytes

### Performance Considerations / Consideraciones de Rendimiento

- Smaller types use less memory and can be faster
- `f64` is preferred over `f32` for most calculations
- `i32` is often the best choice for general-purpose integers
- Use `usize`/`isize` when working with memory indices

### Common Pitfalls / Errores Comunes

1. **Integer Overflow**: Be careful with arithmetic operations that might exceed type limits
2. **Type Mismatches**: Cannot perform operations between different types without explicit casting
3. **Precision Loss**: `f32` has less precision than `f64`
4. **Platform Differences**: `usize`/`isize` size varies between 32-bit and 64-bit systems

## 🔍 Further Reading / Lectura Adicional

- [Rust Book - Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Rust Reference - Types](https://doc.rust-lang.org/reference/types.html)
- [Primitive Types Documentation](https://doc.rust-lang.org/std/index.html#primitives)

## 🧪 Exercises / Ejercicios

1. **Type Exploration**: Try changing the values in the example to see what happens with different ranges
2. **Arithmetic Operations**: Add code to perform various arithmetic operations and print results
3. **Type Conversion**: Experiment with converting between different number types
4. **Overflow Testing**: Test what happens when you exceed the limits of different types

## 📝 Notes / Notas

- This example demonstrates the maximum and minimum values for each type
- In practice, you'll often use `i32` for integers and `f64` for floats
- Rust's type system helps prevent many common programming errors
- Always consider the range of values your program will need when choosing types

---

## 📚 Course Information / Información del Curso

This lesson is part of the Rust Learning Course / Esta lección es parte del Curso de Aprendizaje de Rust
