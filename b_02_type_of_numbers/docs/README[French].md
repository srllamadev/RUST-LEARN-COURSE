# Comment exécuter

```bash
rustc 02_type_of_numbers.rs
./02_type_of_numbers
```

## Langues Disponibles

Cette documentation est également disponible dans d'autres langues :

- [Chinese](docs/README[Chinese].md)
- [Spanish](docs/README[German].md)
- [Spanish](docs/README[Hindi].md)
- [Spanish](docs/README[Portuguese].md)
- [Spanish](docs/README[Russian].md)
- [Spanish](docs/README[Spanish].md)
- [English](../README.md)

# Types Numériques en Rust

Cette leçon démontre les différents types numériques disponibles en Rust, incluant les entiers et les nombres à virgule flottante.

## 📋 Table des Matières

- [🔢 Types Entiers](#-types-entiers)
- [📊 Types à Virgule Flottante](#-types-à-virgule-flottante)
- [🔤 Littéraux Numériques](#-littéraux-numériques)
- [🎯 Inférence de Type](#-inférence-de-type)
- [➗ Opérations Arithmétiques](#-opérations-arithmétiques)
- [🚀 Exécution de l'Exemple](#-exécution-de-lexemple)

## 🔢 Types Entiers

Rust fournit plusieurs types entiers avec différentes tailles et variantes signées/non signées.

### Entiers Non Signés

Les entiers non signés peuvent uniquement représenter des nombres positifs (incluant zéro).

| Type | Taille | Plage | Exemple |
|------|--------|-------|---------|
| `u8` | 8 bits | 0 à 255 | `let x: u8 = 255;` |
| `u16` | 16 bits | 0 à 65 535 | `let x: u16 = 65_535;` |
| `u32` | 32 bits | 0 à 4 294 967 295 | `let x: u32 = 4_294_967_295;` |
| `u64` | 64 bits | 0 à 18 446 744 073 709 551 615 | `let x: u64 = 18_446_744_073_709_551_615;` |
| `u128` | 128 bits | 0 à 340 282 366 920 938 463 463 374 607 431 768 211 455 | `let x: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;` |
| `usize` | Dépendant de l'architecture | 32 bits : 0 à 4 294 967 295 / 64 bits : 0 à 18 446 744 073 709 551 615 | `let x: usize = 100;` |

### Entiers Signés

Les entiers signés peuvent représenter des nombres positifs et négatifs.

| Type | Taille | Plage | Exemple |
|------|--------|-------|---------|
| `i8` | 8 bits | -128 à 127 | `let x: i8 = -128;` |
| `i16` | 16 bits | -32 768 à 32 767 | `let x: i16 = 32_767;` |
| `i32` | 32 bits | -2 147 483 648 à 2 147 483 647 | `let x: i32 = -2_147_483_648;` |
| `i64` | 64 bits | -9 223 372 036 854 775 808 à 9 223 372 036 854 775 807 | `let x: i64 = 9_223_372_036_854_775_807;` |
| `i128` | 128 bits | -170 141 183 460 469 231 731 687 303 715 884 105 728 à 170 141 183 460 469 231 731 687 303 715 884 105 727 | `let x: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728;` |
| `isize` | Dépendant de l'architecture | 32 bits : -2 147 483 648 à 2 147 483 647 / 64 bits : -9 223 372 036 854 775 808 à 9 223 372 036 854 775 807 | `let x: isize = -50;` |

## 📊 Types à Virgule Flottante

Les types à virgule flottante représentent des nombres décimaux avec différents niveaux de précision.

| Type | Taille | Précision | Exemple |
|------|--------|-----------|---------|
| `f32` | 32 bits | ~6-9 chiffres décimaux | `let x: f32 = 3.141592;` |
| `f64` | 64 bits | ~15-17 chiffres décimaux | `let x: f64 = 2.718281828459045;` |

**Note** : `f64` est le type à virgule flottante par défaut en Rust et est généralement préféré pour sa précision supérieure.

## 🔤 Littéraux Numériques

Rust prend en charge diverses façons d'écrire des littéraux numériques pour une meilleure lisibilité.

### Séparateur de Soulignement

```rust
let million = 1_000_000;        // Identique à 1000000
let binaire = 0b_1111_0000;     // Binaire avec séparateurs
let hex = 0x_DEAD_BEEF;         // Hexadécimal avec séparateurs
```

### Différentes Bases

```rust
let decimal = 42;               // Décimal (par défaut)
let hex = 0x2A;                 // Hexadécimal (préfixe 0x)
let octal = 0o52;               // Octal (préfixe 0o)
let binaire = 0b101010;         // Binaire (préfixe 0b)
```

### Suffixes de Type

```rust
let x = 42u32;                  // Suffixe u32
let y = 3.14f32;                // Suffixe f32
let z = 1_000_000i64;           // Suffixe i64
```

## 🎯 Inférence de Type

Rust peut souvent inférer le type d'une variable à partir de son contexte.

```rust
let x = 42;         // i32 (type entier par défaut)
let y = 3.14;       // f64 (type flottant par défaut)
let z = 42u8;       // u8 (annotation de type explicite)
```

## ➗ Opérations Arithmétiques

Tous les types numériques prennent en charge les opérations arithmétiques de base.

```rust
let a = 10;
let b = 3;

let somme = a + b;        // Addition
let difference = a - b;   // Soustraction
let produit = a * b;      // Multiplication
let quotient = a / b;     // Division
let reste = a % b;        // Modulo
```

### Comportement de Dépassement

- **Mode debug** : Le programme panique en cas de dépassement d'entier
- **Mode release** : Retourne à zéro (complément à deux)

```rust
let x: u8 = 255;
let y = x + 1;  // Panique en debug, retourne à 0 en release
```

## 🚀 Exécution de l'Exemple

### Prérequis

Assurez-vous que Rust est installé sur votre système.

```bash
# Vérifier si Rust est installé
rustc --version
cargo --version
```

### Compilation et Exécution

```bash
# Naviguer vers le répertoire du projet
cd b_02_type_of_numbers

# Compiler le programme
rustc 02_type_of_numbers.rs

# Exécuter l'exécutable
./02_type_of_numbers.exe  # Sur Windows
# ou
./02_type_of_numbers      # Sur Linux/macOS
```

### Sortie Attendue

```text
Entiers non signés : u8=255 u16=65535 u32=4294967295 u64=18446744073709551615 u128=340282366920938463463374607431768211455 usize=100
Entiers signés : i8=-128 i16=32767 i32=-2147483648 i64=9223372036854775807 i128=-170141183460469231731687303715884105728 isize=-50
Nombres flottants : f32=3.141592 f64=2.718281828459045
Ceci est imprimé sur la même ligne !
```

## 📚 Concepts Clés

### Utilisation de la Mémoire

- `u8` et `i8` : 1 octet
- `u16` et `i16` : 2 octets
- `u32` et `i32` : 4 octets
- `u64` et `i64` : 8 octets
- `u128` et `i128` : 16 octets
- `f32` : 4 octets
- `f64` : 8 octets

### Considérations de Performance

- Les types plus petits utilisent moins de mémoire et peuvent être plus rapides
- `f64` est préféré à `f32` pour la plupart des calculs
- `i32` est souvent le meilleur choix pour les entiers à usage général
- Utilisez `usize`/`isize` lorsque vous travaillez avec des indices de mémoire

### Pièges Courants

1. **Dépassement d'Entier** : Soyez prudent avec les opérations arithmétiques qui pourraient dépasser les limites du type
2. **Incompatibilité de Types** : Impossible d'effectuer des opérations entre différents types sans conversion explicite
3. **Perte de Précision** : `f32` a moins de précision que `f64`
4. **Différences de Plateforme** : La taille de `usize`/`isize` varie entre les systèmes 32 bits et 64 bits

## 🔍 Lectures Supplémentaires

- [Livre Rust - Types de Données](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Référence Rust - Types](https://doc.rust-lang.org/reference/types.html)
- [Documentation des Types Primitifs](https://doc.rust-lang.org/std/index.html#primitives)

## 🧪 Exercices

1. **Exploration des Types** : Essayez de changer les valeurs dans l'exemple pour voir ce qui se passe avec différentes plages
2. **Opérations Arithmétiques** : Ajoutez du code pour effectuer diverses opérations arithmétiques et afficher les résultats
3. **Conversion de Type** : Expérimentez la conversion entre différents types numériques
4. **Test de Dépassement** : Testez ce qui se passe lorsque vous dépassez les limites de différents types

## 📝 Notes

- Cet exemple démontre les valeurs maximales et minimales pour chaque type
- En pratique, vous utiliserez souvent `i32` pour les entiers et `f64` pour les flottants
- Le système de types de Rust aide à prévenir de nombreuses erreurs de programmation courantes
- Considérez toujours la plage de valeurs dont votre programme aura besoin lors du choix des types

---

## 🌍 Langues Disponibles

Cette documentation est également disponible dans d'autres langues :

- [Spanish](README[Spanish].md)
- [Hindi](README[Hindi].md)
- [Chinese](README[Chinese].md)

## 📚 Informations sur le Cours

Cette leçon fait partie du Cours d'Apprentissage Rust
