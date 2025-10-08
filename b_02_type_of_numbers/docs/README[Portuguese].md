# Como executar

```bash
rustc 02_type_of_numbers.rs
./02_type_of_numbers
```

## Idiomas Disponíveis

Esta documentação também está disponível em outros idiomas:

- [Spanish](docs/README[Spanish].md)
- [Hindi](docs/README[Hindi].md)
- [Chinese](docs/README[Chinese].md)

# Tipos Numéricos em Rust

Esta lição demonstra os diferentes tipos numéricos disponíveis em Rust, incluindo inteiros e números de ponto flutuante.

## 📋 Índice

- [🔢 Tipos Inteiros](#-tipos-inteiros)
- [📊 Tipos de Ponto Flutuante](#-tipos-de-ponto-flutuante)
- [🔤 Literais Numéricos](#-literais-numéricos)
- [🎯 Inferência de Tipo](#-inferência-de-tipo)
- [➗ Operações Aritméticas](#-operações-aritméticas)
- [🚀 Executando o Exemplo](#-executando-o-exemplo)

## 🔢 Tipos Inteiros

Rust fornece vários tipos inteiros com diferentes tamanhos e variantes com/semsinal.

### Inteiros Sem Sinal

Inteiros sem sinal podem representar apenas números positivos (incluindo zero).

| Tipo | Tamanho | Intervalo | Exemplo |
|------|---------|-----------|---------|
| `u8` | 8 bits | 0 a 255 | `let x: u8 = 255;` |
| `u16` | 16 bits | 0 a 65.535 | `let x: u16 = 65_535;` |
| `u32` | 32 bits | 0 a 4.294.967.295 | `let x: u32 = 4_294_967_295;` |
| `u64` | 64 bits | 0 a 18.446.744.073.709.551.615 | `let x: u64 = 18_446_744_073_709_551_615;` |
| `u128` | 128 bits | 0 a 340.282.366.920.938.463.463.374.607.431.768.211.455 | `let x: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;` |
| `usize` | Dependente da arquitetura | 32 bits: 0 a 4.294.967.295 / 64 bits: 0 a 18.446.744.073.709.551.615 | `let x: usize = 100;` |

### Inteiros Com Sinal

Inteiros com sinal podem representar números positivos e negativos.

| Tipo | Tamanho | Intervalo | Exemplo |
|------|---------|-----------|---------|
| `i8` | 8 bits | -128 a 127 | `let x: i8 = -128;` |
| `i16` | 16 bits | -32.768 a 32.767 | `let x: i16 = 32_767;` |
| `i32` | 32 bits | -2.147.483.648 a 2.147.483.647 | `let x: i32 = -2_147_483_648;` |
| `i64` | 64 bits | -9.223.372.036.854.775.808 a 9.223.372.036.854.775.807 | `let x: i64 = 9_223_372_036_854_775_807;` |
| `i128` | 128 bits | -170.141.183.460.469.231.731.687.303.715.884.105.728 a 170.141.183.460.469.231.731.687.303.715.884.105.727 | `let x: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728;` |
| `isize` | Dependente da arquitetura | 32 bits: -2.147.483.648 a 2.147.483.647 / 64 bits: -9.223.372.036.854.775.808 a 9.223.372.036.854.775.807 | `let x: isize = -50;` |

## 📊 Tipos de Ponto Flutuante

Tipos de ponto flutuante representam números decimais com diferentes níveis de precisão.

| Tipo | Tamanho | Precisão | Exemplo |
|------|---------|----------|---------|
| `f32` | 32 bits | ~6-9 dígitos decimais | `let x: f32 = 3.141592;` |
| `f64` | 64 bits | ~15-17 dígitos decimais | `let x: f64 = 2.718281828459045;` |

**Nota**: `f64` é o tipo de ponto flutuante padrão em Rust e é geralmente preferido por sua maior precisão.

## 🔤 Literais Numéricos

Rust suporta várias maneiras de escrever literais numéricos para melhor legibilidade.

### Separador de Sublinhado

```rust
let milhao = 1_000_000;        // Mesmo que 1000000
let binario = 0b_1111_0000;    // Binário com separadores
let hex = 0x_DEAD_BEEF;        // Hexadecimal com separadores
```

### Diferentes Bases

```rust
let decimal = 42;              // Decimal (padrão)
let hex = 0x2A;                // Hexadecimal (prefixo 0x)
let octal = 0o52;              // Octal (prefixo 0o)
let binario = 0b101010;        // Binário (prefixo 0b)
```

### Sufixos de Tipo

```rust
let x = 42u32;                 // Sufixo u32
let y = 3.14f32;               // Sufixo f32
let z = 1_000_000i64;          // Sufixo i64
```

## 🎯 Inferência de Tipo

Rust pode frequentemente inferir o tipo de uma variável a partir de seu contexto.

```rust
let x = 42;        // i32 (tipo inteiro padrão)
let y = 3.14;      // f64 (tipo float padrão)
let z = 42u8;      // u8 (anotação de tipo explícita)
```

## ➗ Operações Aritméticas

Todos os tipos numéricos suportam operações aritméticas básicas.

```rust
let a = 10;
let b = 3;

let soma = a + b;        // Adição
let diferenca = a - b;   // Subtração
let produto = a * b;     // Multiplicação
let quociente = a / b;   // Divisão
let resto = a % b;       // Módulo
```

### Comportamento de Overflow

- **Modo debug**: Programa entra em pânico com overflow de inteiro
- **Modo release**: Volta ao início (complemento de dois)

```rust
let x: u8 = 255;
let y = x + 1;  // Entra em pânico no debug, volta a 0 no release
```

## 🚀 Executando o Exemplo

### Pré-requisitos

Certifique-se de que Rust está instalado em seu sistema.

```bash
# Verificar se Rust está instalado
rustc --version
cargo --version
```

### Compilação e Execução

```bash
# Navegar para o diretório do projeto
cd b_02_type_of_numbers

# Compilar o programa
rustc 02_type_of_numbers.rs

# Executar o executável
./02_type_of_numbers.exe  # No Windows
# ou
./02_type_of_numbers      # No Linux/macOS
```

### Saída Esperada

```text
Inteiros sem sinal: u8=255 u16=65535 u32=4294967295 u64=18446744073709551615 u128=340282366920938463463374607431768211455 usize=100
Inteiros com sinal: i8=-128 i16=32767 i32=-2147483648 i64=9223372036854775807 i128=-170141183460469231731687303715884105728 isize=-50
Números flutuantes: f32=3.141592 f64=2.718281828459045
Isto é impresso na mesma linha!
```

## 📚 Conceitos Chave

### Uso de Memória

- `u8` e `i8`: 1 byte
- `u16` e `i16`: 2 bytes
- `u32` e `i32`: 4 bytes
- `u64` e `i64`: 8 bytes
- `u128` e `i128`: 16 bytes
- `f32`: 4 bytes
- `f64`: 8 bytes

### Considerações de Performance

- Tipos menores usam menos memória e podem ser mais rápidos
- `f64` é preferido a `f32` para a maioria dos cálculos
- `i32` é frequentemente a melhor escolha para inteiros de uso geral
- Use `usize`/`isize` ao trabalhar com índices de memória

### Armadilhas Comuns

1. **Overflow de Inteiro**: Tenha cuidado com operações aritméticas que podem exceder os limites do tipo
2. **Incompatibilidades de Tipo**: Não é possível realizar operações entre tipos diferentes sem conversão explícita
3. **Perda de Precisão**: `f32` tem menos precisão que `f64`
4. **Diferenças de Plataforma**: O tamanho de `usize`/`isize` varia entre sistemas 32-bit e 64-bit

## 🔍 Leitura Adicional

- [Livro do Rust - Tipos de Dados](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Referência do Rust - Tipos](https://doc.rust-lang.org/reference/types.html)
- [Documentação dos Tipos Primitivos](https://doc.rust-lang.org/std/index.html#primitives)

## 🧪 Exercícios

1. **Exploração de Tipos**: Tente alterar os valores no exemplo para ver o que acontece com diferentes intervalos
2. **Operações Aritméticas**: Adicione código para realizar várias operações aritméticas e imprimir resultados
3. **Conversão de Tipo**: Experimente converter entre diferentes tipos numéricos
4. **Teste de Overflow**: Teste o que acontece quando você excede os limites de diferentes tipos

## 📝 Notas

- Este exemplo demonstra os valores máximo e mínimo para cada tipo
- Na prática, você frequentemente usará `i32` para inteiros e `f64` para floats
- O sistema de tipos de Rust ajuda a prevenir muitos erros comuns de programação
- Sempre considere o intervalo de valores que seu programa precisa ao escolher tipos

---

## 🌍 Idiomas Disponíveis

Esta documentação também está disponível em outros idiomas:

- [Spanish](README[Spanish].md)
- [Hindi](README[Hindi].md)
- [Chinese](README[Chinese].md)

## 📚 Informações do Curso

Esta lição faz parte do Curso de Aprendizado Rust
