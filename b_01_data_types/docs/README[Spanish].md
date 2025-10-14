# Cómo ejecutar

Ve al directorio del proyecto y ejecuta:

```bash
cargo run
```

## Idiomas Disponibles

Esta documentación también está disponible en otros idiomas:

- [Chinese](docs/README[Chinese].md)
- [Hindi](docs/README[French].md)
- [Spanish](docs/README[German].md)
- [Spanish](docs/README[Hindi].md)
- [Spanish](docs/README[Portuguese].md)
- [Spanish](docs/README[Russian].md)
- [Spanish](docs/README[Spanish].md)
- [English](../README.md)

# Tipos de Datos y Propiedad en Rust

Esta lección introduce los tipos de datos de Rust y los conceptos fundamentales de propiedad y préstamo. Comprender estos conceptos es crucial para escribir código Rust seguro y eficiente.

## Índice

- [🦙 Tipos Escalares](#-tipos-escalares)
- [🦙 Tipos Compuestos](#-tipos-compuestos)
- [🦙 Propiedad](#-propiedad)
- [🦙 Referencias y Préstamo](#-referencias-y-préstamo)
- [🚀 Ejecutando el Ejemplo](#-ejecutando-el-ejemplo)
- [🦙 Conceptos Clave](#-conceptos-clave)
- [🦙 Ejercicios](#-ejercicios)

## 🦙 Tipos Escalares

Los tipos escalares representan valores únicos. Rust tiene cuatro tipos escalares primarios: enteros, números de punto flotante, booleanos y caracteres.

### Enteros

Rust proporciona varios tipos enteros con diferentes tamaños y variantes con/sin signo:

```rust
let x: i32 = -42;    // Entero de 32 bits con signo
let y: u64 = 42;     // Entero de 64 bits sin signo
```

### Números de Punto Flotante

Rust soporta dos tipos de punto flotante:

```rust
let f: f64 = 3.1415; // Número de punto flotante de 64 bits
```

### Booleanos

Los valores booleanos representan verdadero o falso:

```rust
let b: bool = true;  // Valor booleano
```

### Caracteres

Los caracteres representan valores escalares Unicode únicos:

```rust
let c: char = 'λ';   // Carácter Unicode (lambda)
```

## 📦 Tipos Compuestos

Los tipos compuestos agrupan múltiples valores en un tipo.

### Tuplas

Las tuplas son colecciones de tamaño fijo que pueden contener tipos mixtos:

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);  // Declaración de tupla
let (a, b, c) = tup;                       // Desestructuración
println!("tupla: ({}, {}, {})", a, b, c);  // Accediendo a elementos
```

### Arrays

Los arrays son colecciones de tamaño fijo de elementos del mismo tipo:

```rust
let arr: [i32; 4] = [1, 2, 3, 4];  // Declaración de array
println!("array: {:?}", arr);       // Imprimir array completo
```

### Rebanadas

Las rebanadas proporcionan una vista en arrays sin tomar propiedad:

```rust
fn print_slice(slice: &[i32]) {
    println!("rebanada: {:?}", slice);
}
print_slice(&arr);  // Pasar array como rebanada
```

## 🦙 Propiedad

La propiedad es la característica más única de Rust y permite seguridad de memoria sin un recolector de basura.

### Reglas de Propiedad

1. **Cada valor tiene una variable que es su propietaria**
2. **Cada valor puede tener solo una propietaria a la vez**
3. **Cuando la propietaria sale del alcance, el valor es descartado**

```rust
let s = String::from("hello");  // s es propietaria del String
takes_ownership(s);             // s es movida a la función
// s ya no es válida aquí
```

### Mover vs Copiar

- **Mover**: Transfiere propiedad (para datos asignados en el heap como `String`)
- **Copiar**: Crea un duplicado (para datos solo en la pila como enteros)

```rust
let x = 5;        // i32 implementa Copy
let y = x;        // x es copiada, ambas son válidas
println!("x: {}, y: {}", x, y);  // Funciona bien

let s1 = String::from("hello");  // String no implementa Copy
let s2 = s1;                     // s1 es movida a s2
// println!("{}", s1);           // Esto no compilaría
```

## 🦙 Referencias y Préstamo

Las referencias permiten acceder a datos sin tomar propiedad.

### Referencias Inmutables

```rust
let s2 = String::from("world");
borrow_string(&s2);              // Prestar s2 inmutablemente
println!("s2 después del préstamo: {}", s2);  // s2 aún es válida
```

### Referencias Mutables

```rust
fn change_string(s: &mut String) {
    s.push_str(" world");
}

let mut s = String::from("hello");
change_string(&mut s);
println!("{}", s);  // Imprime "hello world"
```

### Reglas de Referencia

1. **Puedes tener una referencia mutable O múltiples referencias inmutables**
2. **Las referencias deben ser siempre válidas**

## 🚀 Ejecutando el Ejemplo

### Prerrequisitos

Asegúrate de que Rust esté instalado en tu sistema.

```bash
# Verificar si Rust está instalado
rustc --version
cargo --version
```

### Compilación y Ejecución

```bash
# Navegar al directorio del proyecto
cd a_02_data_types

# Ejecutar con Cargo (recomendado)
cargo run

# O compilar manualmente
cargo build
./target/debug/a_02_data_types.exe  # En Windows
# o
./target/debug/a_02_data_types      # En Linux/macOS
```

### Salida Esperada

```text
i32: -42 | u64: 42 | f64: 3.1415 | bool: true | char: λ
tupla: (500, 6.4, 1)
array: [1, 2, 3, 4]
rebanada: [1, 2, 3, 4]
Tomé propiedad de: hello
Presté: world
s2 después del préstamo: world
```

## 🦙 Conceptos Clave

### Gestión de Memoria

- **Pila**: Rápida, asignación/desasignación automática, tamaño fijo
- **Heap**: Más lenta, asignación manual, tamaño variable
- **Propiedad**: Garantiza seguridad de memoria y previene fugas de memoria

### Beneficios del Préstamo

- **Abstracciones de costo cero**: Las referencias no añaden sobrecarga de tiempo de ejecución
- **Previene carreras de datos**: Las reglas de préstamo mutable previenen modificación concurrente
- **Flexible**: Puede prestarse inmutablemente múltiples veces o mutablemente una vez

### Patrones Comunes

1. **Usa referencias cuando no necesites propiedad**
2. **Usa `&mut` cuando necesites modificar datos prestados**
3. **Devuelve valores propios cuando el llamador debe tomar propiedad**
4. **Usa rebanadas para acceso flexible a arrays**

### Implicaciones de Rendimiento

- **Tipos Copy** (enteros, flotantes, booleanos, caracteres): Baratos de pasar
- **Tipos Move** (String, Vec): Transfiere propiedad cuando sea posible
- **Tipos Borrow**: Usa referencias para evitar copias innecesarias

## 🌚 Ejercicios

1. **Exploración de Tipos**: Experimenta con diferentes tipos escalares y sus rangos
2. **Operaciones con Tuplas**: Crea tuplas con diferentes combinaciones de tipos y practica desestructuración
3. **Manipulación de Arrays**: Crea arrays de diferentes tamaños y accede a elementos usando indexación
4. **Transferencia de Propiedad**: Escribe funciones que toman propiedad vs. parámetros de préstamo
5. **Práctica con Referencias**: Crea funciones que aceptan referencias mutables e inmutables
6. **Operaciones con Rebanadas**: Practica creando rebanadas de arrays y pasándolas a funciones

## Lectura Adicional

- [Libro de Rust - Tipos de Datos](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Libro de Rust - Propiedad](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Libro de Rust - Referencias y Préstamo](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Referencia de Rust - Tipos](https://doc.rust-lang.org/reference/types.html)

## Notas

- **Propiedad** es el arma secreta de Rust para seguridad de memoria
- **Préstamo** permite código eficiente sin sacrificar seguridad
- **Tipos** determinan si los valores son copiados o movidos
- Practica estos conceptos regularmente - son fundamentales para la programación en Rust
