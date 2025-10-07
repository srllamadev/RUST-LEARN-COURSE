## Como ejecutar

```bash
rustc 02_type_of_numbers.rs
./02_type_of_numbers
```

## Tenemos los siguientes idiomas disponibles

- [Chinese](README[Chinese].md)
- [Hindi](README[Hindi].md)
- [English](../README.md)

# Tipos de Números en Rust

Esta lección demuestra los diferentes tipos de números disponibles en Rust, incluyendo enteros y números de punto flotante.

## 📋 Tabla de Contenidos

- [🔢 Tipos Enteros](#-tipos-enteros)
- [📊 Tipos de Punto Flotante](#-tipos-de-punto-flotante)
- [🔤 Literales Numéricos](#-literales-numéricos)
- [🎯 Inferencia de Tipos](#-inferencia-de-tipos)
- [➗ Operaciones Aritméticas](#-operaciones-aritméticas)
- [🚀 Ejecutar el Ejemplo](#-ejecutar-el-ejemplo)

## 🔢 Tipos Enteros

Rust proporciona varios tipos enteros con diferentes tamaños y variantes con/sin signo.

### Enteros Sin Signo

Los enteros sin signo solo pueden representar números positivos (incluyendo cero).

| Tipo | Tamaño | Rango | Ejemplo |
|------|--------|-------|---------|
| `u8` | 8 bits | 0 a 255 | `let x: u8 = 255;` |
| `u16` | 16 bits | 0 a 65,535 | `let x: u16 = 65_535;` |
| `u32` | 32 bits | 0 a 4,294,967,295 | `let x: u32 = 4_294_967_295;` |
| `u64` | 64 bits | 0 a 18,446,744,073,709,551,615 | `let x: u64 = 18_446_744_073_709_551_615;` |
| `u128` | 128 bits | 0 a 340,282,366,920,938,463,463,374,607,431,768,211,455 | `let x: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;` |
| `usize` | Dependiente de la arquitectura | 32-bit: 0 a 4,294,967,295 / 64-bit: 0 a 18,446,744,073,709,551,615 | `let x: usize = 100;` |

### Enteros Con Signo

Los enteros con signo pueden representar tanto números positivos como negativos.

| Tipo | Tamaño | Rango | Ejemplo |
|------|--------|-------|---------|
| `i8` | 8 bits | -128 a 127 | `let x: i8 = -128;` |
| `i16` | 16 bits | -32,768 a 32,767 | `let x: i16 = 32_767;` |
| `i32` | 32 bits | -2,147,483,648 a 2,147,483,647 | `let x: i32 = -2_147_483_648;` |
| `i64` | 64 bits | -9,223,372,036,854,775,808 a 9,223,372,036,854,775,807 | `let x: i64 = 9_223_372_036_854_775_807;` |
| `i128` | 128 bits | -170,141,183,460,469,231,731,687,303,715,884,105,728 a 170,141,183,460,469,231,731,687,303,715,884,105,727 | `let x: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728;` |
| `isize` | Dependiente de la arquitectura | 32-bit: -2,147,483,648 a 2,147,483,647 / 64-bit: -9,223,372,036,854,775,808 a 9,223,372,036,854,775,807 | `let x: isize = -50;` |

## 📊 Tipos de Punto Flotante

Los tipos de punto flotante representan números decimales con diferentes niveles de precisión.

| Tipo | Tamaño | Precisión | Ejemplo |
|------|--------|-----------|---------|
| `f32` | 32 bits | ~6-9 dígitos decimales | `let x: f32 = 3.141592;` |
| `f64` | 64 bits | ~15-17 dígitos decimales | `let x: f64 = 2.718281828459045;` |

**Nota**: `f64` es el tipo de punto flotante predeterminado en Rust y generalmente se prefiere por su mayor precisión.

## 🔤 Literales Numéricos

Rust soporta varias formas de escribir literales numéricos para mejor legibilidad.

### Separador de Guion Bajo

```rust
let million = 1_000_000;        // Igual a 1000000
let binary = 0b_1111_0000;      // Binario con separadores
let hex = 0x_DEAD_BEEF;         // Hexadecimal con separadores
```

### Diferentes Bases

```rust
let decimal = 42;               // Decimal (predeterminado)
let hex = 0x2A;                 // Hexadecimal (prefijo 0x)
let octal = 0o52;               // Octal (prefijo 0o)
let binary = 0b101010;          // Binario (prefijo 0b)
```

### Sufijos de Tipo

```rust
let x = 42u32;                  // Sufijo u32
let y = 3.14f32;                // Sufijo f32
let z = 1_000_000i64;           // Sufijo i64
```

## 🎯 Inferencia de Tipos

Rust puede inferir el tipo de una variable desde su contexto.

```rust
let x = 42;         // i32 (tipo entero predeterminado)
let y = 3.14;       // f64 (tipo float predeterminado)
let z = 42u8;       // u8 (anotación de tipo explícita)
```

## ➗ Operaciones Aritméticas

Todos los tipos numéricos soportan operaciones aritméticas básicas.

```rust
let a = 10;
let b = 3;

let sum = a + b;        // Suma
let difference = a - b; // Resta
let product = a * b;    // Multiplicación
let quotient = a / b;   // División
let remainder = a % b;  // Módulo
```

### Comportamiento de Desbordamiento

- **Modo debug**: El programa entra en pánico con desbordamiento de enteros
- **Modo release**: Se envuelve (complemento a dos)

```rust
let x: u8 = 255;
let y = x + 1;  // Entra en pánico en debug, se envuelve a 0 en release
```

## 🚀 Ejecutar el Ejemplo

### Prerrequisitos

Asegúrate de tener Rust instalado en tu sistema.

```bash
# Verificar si Rust está instalado
rustc --version
cargo --version
```

### Compilación y Ejecución

```bash
# Navegar al directorio del proyecto
cd b_02_type_of_numbers

# Compilar el programa
rustc 02_type_of_numbers.rs

# Ejecutar el ejecutable
./02_type_of_numbers.exe  # En Windows
# o
./02_type_of_numbers      # En Linux/macOS
```

### Salida Esperada

```text
Unsigned integers: u8=255 u16=65535 u32=4294967295 u64=18446744073709551615 u128=340282366920938463463374607431768211455 usize=100
Signed integers: i8=-128 i16=32767 i32=-2147483648 i64=9223372036854775807 i128=-170141183460469231731687303715884105728 isize=-50
Floating numbers: f32=3.141592 f64=2.718281828459045
This is printed on the same line!
```

## 📚 Conceptos Clave

### Uso de Memoria

- `u8` e `i8`: 1 byte
- `u16` e `i16`: 2 bytes
- `u32` e `i32`: 4 bytes
- `u64` e `i64`: 8 bytes
- `u128` e `i128`: 16 bytes
- `f32`: 4 bytes
- `f64`: 8 bytes

### Consideraciones de Rendimiento

- Los tipos más pequeños usan menos memoria y pueden ser más rápidos
- `f64` se prefiere sobre `f32` para la mayoría de los cálculos
- `i32` es a menudo la mejor opción para enteros de propósito general
- Usa `usize`/`isize` cuando trabajes con índices de memoria

### Errores Comunes

1. **Desbordamiento de Enteros**: Ten cuidado con operaciones aritméticas que puedan exceder los límites del tipo
2. **Desajustes de Tipo**: No se puede realizar operaciones entre diferentes tipos sin conversión explícita
3. **Pérdida de Precisión**: `f32` tiene menos precisión que `f64`
4. **Diferencias de Plataforma**: El tamaño de `usize`/`isize` varía entre sistemas de 32-bit y 64-bit

## 🔍 Lectura Adicional

- [Libro de Rust - Tipos de Datos](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Referencia de Rust - Tipos](https://doc.rust-lang.org/reference/types.html)
- [Documentación de Tipos Primitivos](https://doc.rust-lang.org/std/index.html#primitives)

## 🧪 Ejercicios

1. **Exploración de Tipos**: Intenta cambiar los valores en el ejemplo para ver qué sucede con diferentes rangos
2. **Operaciones Aritméticas**: Agrega código para realizar varias operaciones aritméticas e imprimir resultados
3. **Conversión de Tipo**: Experimenta convirtiendo entre diferentes tipos numéricos
4. **Pruebas de Desbordamiento**: Prueba qué sucede cuando excedes los límites de diferentes tipos

## 📝 Notas

- Este ejemplo demuestra los valores máximo y mínimo para cada tipo
- En la práctica, a menudo usarás `i32` para enteros y `f64` para floats
- El sistema de tipos de Rust ayuda a prevenir muchos errores comunes de programación
- Siempre considera el rango de valores que tu programa necesitará al elegir tipos

---

## 🌍 Idiomas Disponibles

Esta documentación también está disponible en otros idiomas:

- [English](README.md)
- [हिंदी](README[Hindi].md)
- [中文](README[Chinese].md)

## 📚 Información del Curso

Esta lección es parte del Curso de Aprendizaje de Rust
