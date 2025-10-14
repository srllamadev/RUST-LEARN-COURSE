# Como executar

Vá para o diretório do projeto e execute:

```bash
cargo run
```

## Idiomas Disponíveis

Esta documentação também está disponível em outros idiomas:

- [Chinese](docs/README[Chinese].md)
- [Hindi](docs/README[French].md)
- [Spanish](docs/README[German].md)
- [Spanish](docs/README[Hindi].md)
- [Spanish](docs/README[Portuguese].md)
- [Spanish](docs/README[Russian].md)
- [Spanish](docs/README[Spanish].md)
- [English](../README.md)

# Tipos de Dados e Propriedade em Rust

Esta lição apresenta os tipos de dados do Rust e os conceitos fundamentais de propriedade e empréstimo. Compreender esses conceitos é crucial para escrever código Rust seguro e eficiente.

## Índice

- [🦙 Tipos Escalares](#-tipos-escalares)
- [🦙 Tipos Compostos](#-tipos-compostos)
- [🦙 Propriedade](#-propriedade)
- [🦙 Referências e Empréstimo](#-referências-e-empréstimo)
- [🚀 Executando o Exemplo](#-executando-o-exemplo)
- [🦙 Conceitos Chave](#-conceitos-chave)
- [🦙 Exercícios](#-exercícios)

## 🦙 Tipos Escalares

Tipos escalares representam valores únicos. Rust tem quatro tipos escalares primários: inteiros, números de ponto flutuante, booleanos e caracteres.

### Inteiros

Rust fornece vários tipos inteiros com diferentes tamanhos e variantes com/semsinal:

```rust
let x: i32 = -42;    // Inteiro de 32 bits com sinal
let y: u64 = 42;     // Inteiro de 64 bits sem sinal
```

### Números de Ponto Flutuante

Rust suporta dois tipos de ponto flutuante:

```rust
let f: f64 = 3.1415; // Número de ponto flutuante de 64 bits
```

### Booleanos

Valores booleanos representam verdadeiro ou falso:

```rust
let b: bool = true;  // Valor booleano
```

### Caracteres

Caracteres representam valores escalares Unicode únicos:

```rust
let c: char = 'λ';   // Caractere Unicode (lambda)
```

## 📦 Tipos Compostos

Tipos compostos agrupam múltiplos valores em um tipo.

### Tuplas

Tuplas são coleções de tamanho fixo que podem conter tipos mistos:

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);  // Declaração de tupla
let (a, b, c) = tup;                       // Desestruturação
println!("tupla: ({}, {}, {})", a, b, c);  // Acessando elementos
```

### Arrays

Arrays são coleções de tamanho fixo de elementos do mesmo tipo:

```rust
let arr: [i32; 4] = [1, 2, 3, 4];  // Declaração de array
println!("array: {:?}", arr);       // Imprimir array completo
```

### Fatias

Fatias fornecem uma visão em arrays sem assumir propriedade:

```rust
fn print_slice(slice: &[i32]) {
    println!("fatia: {:?}", slice);
}
print_slice(&arr);  // Passar array como fatia
```

## 🦙 Propriedade

Propriedade é a característica mais única do Rust e permite segurança de memória sem um coletor de lixo.

### Regras de Propriedade

1. **Cada valor tem uma variável que é sua proprietária**
2. **Cada valor pode ter apenas um proprietário por vez**
3. **Quando a proprietária sai do escopo, o valor é descartado**

```rust
let s = String::from("hello");  // s é proprietária da String
takes_ownership(s);             // s é movida para a função
// s não é mais válida aqui
```

### Mover vs Copiar

- **Mover**: Transfere propriedade (para dados alocados no heap como `String`)
- **Copiar**: Cria um duplicado (para dados apenas na pilha como inteiros)

```rust
let x = 5;        // i32 implementa Copy
let y = x;        // x é copiada, ambas são válidas
println!("x: {}, y: {}", x, y);  // Funciona bem

let s1 = String::from("hello");  // String não implementa Copy
let s2 = s1;                     // s1 é movida para s2
// println!("{}", s1);           // Isso não compilaria
```

## 🦙 Referências e Empréstimo

Referências permitem acessar dados sem assumir propriedade.

### Referências Imutáveis

```rust
let s2 = String::from("world");
borrow_string(&s2);              // Empréstimo imutável de s2
println!("s2 após empréstimo: {}", s2);  // s2 ainda é válida
```

### Referências Mutáveis

```rust
fn change_string(s: &mut String) {
    s.push_str(" world");
}

let mut s = String::from("hello");
change_string(&mut s);
println!("{}", s);  // Imprime "hello world"
```

### Regras de Referência

1. **Você pode ter uma referência mutável OU múltiplas referências imutáveis**
2. **Referências devem sempre ser válidas**

## 🚀 Executando o Exemplo

### Pré-requisitos

Certifique-se de que Rust está instalado no seu sistema.

```bash
# Verificar se Rust está instalado
rustc --version
cargo --version
```

### Compilação e Execução

```bash
# Navegar para o diretório do projeto
cd a_02_data_types

# Executar com Cargo (recomendado)
cargo run

# Ou compilar manualmente
cargo build
./target/debug/a_02_data_types.exe  # No Windows
# ou
./target/debug/a_02_data_types      # No Linux/macOS
```

### Saída Esperada

```text
i32: -42 | u64: 42 | f64: 3.1415 | bool: true | char: λ
tupla: (500, 6.4, 1)
array: [1, 2, 3, 4]
fatia: [1, 2, 3, 4]
Eu assumi propriedade de: hello
Eu emprestei: world
s2 após empréstimo: world
```

## 🦙 Conceitos Chave

### Gerenciamento de Memória

- **Pilha**: Rápida, alocação/desalocação automática, tamanho fixo
- **Heap**: Mais lenta, alocação manual, tamanho variável
- **Propriedade**: Garante segurança de memória e previne vazamentos de memória

### Benefícios do Empréstimo

- **Abstrações de custo zero**: Referências não adicionam overhead de runtime
- **Previne corridas de dados**: Regras de empréstimo mutável previnem modificação concorrente
- **Flexível**: Pode emprestar imutavelmente múltiplas vezes ou mutavelmente uma vez

### Padrões Comuns

1. **Use referências quando não precisar de propriedade**
2. **Use `&mut` quando precisar modificar dados emprestados**
3. **Retorne valores próprios quando o chamador deve assumir propriedade**
4. **Use fatias para acesso flexível a arrays**

### Implicações de Performance

- **Tipos Copy** (inteiros, floats, booleanos, caracteres): Baratos de passar
- **Tipos Move** (String, Vec): Transfira propriedade quando possível
- **Tipos Borrow**: Use referências para evitar cópias desnecessárias

## 🌚 Exercícios

1. **Exploração de Tipos**: Experimente diferentes tipos escalares e seus intervalos
2. **Operações com Tuplas**: Crie tuplas com diferentes combinações de tipos e pratique desestruturação
3. **Manipulação de Arrays**: Crie arrays de diferentes tamanhos e acesse elementos usando indexação
4. **Transferência de Propriedade**: Escreva funções que assumem propriedade vs. parâmetros emprestados
5. **Prática com Referências**: Crie funções que aceitam referências mutáveis e imutáveis
6. **Operações com Fatias**: Pratique criando fatias de arrays e passá-las para funções

## Leitura Adicional

- [Livro do Rust - Tipos de Dados](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Livro do Rust - Propriedade](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Livro do Rust - Referências e Empréstimo](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Referência do Rust - Tipos](https://doc.rust-lang.org/reference/types.html)

## Notas

- **Propriedade** é a arma secreta do Rust para segurança de memória
- **Empréstimo** permite código eficiente sem sacrificar segurança
- **Tipos** determinam se valores são copiados ou movidos
- Pratique esses conceitos regularmente - eles são fundamentais para programação em Rust
