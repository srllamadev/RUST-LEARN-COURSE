# Comment exécuter

Allez dans le répertoire du projet et exécutez :

```bash
cargo run
```

## Langues Disponibles

Cette documentation est également disponible dans d'autres langues :

- [Chinese](docs/README[Chinese].md)
- [Hindi](docs/README[French].md)
- [Spanish](docs/README[German].md)
- [Spanish](docs/README[Hindi].md)
- [Spanish](docs/README[Portuguese].md)
- [Spanish](docs/README[Russian].md)
- [Spanish](docs/README[Spanish].md)
- [English](../README.md)

## Types de Données et Propriété en Rust

Ce cours présente les types de données de Rust et les concepts fondamentaux de propriété et d'emprunt. Comprendre ces concepts est crucial pour écrire du code Rust sûr et efficace.

## Table des Matières

- [🦙 Types Scalaires](#-types-scalaires)
- [🦙 Types Composés](#-types-composés)
- [🦙 Propriété](#-propriété)
- [🦙 Références et Emprunt](#-références-et-emprunt)
- [🚀 Exécution de l'Exemple](#-exécution-de-lexemple)
- [🦙 Concepts Clés](#-concepts-clés)
- [🦙 Exercices](#-exercices)

## 🦙 Types Scalaires

Les types scalaires représentent des valeurs uniques. Rust a quatre types scalaires principaux : les entiers, les nombres à virgule flottante, les booléens et les caractères.

### Entiers

Rust fournit divers types d'entiers avec différentes tailles et variantes signées/non signées :

```rust
let x: i32 = -42;    // Entier signé 32 bits
let y: u64 = 42;     // Entier non signé 64 bits
```

### Nombres à Virgule Flottante

Rust prend en charge deux types de nombres à virgule flottante :

```rust
let f: f64 = 3.1415; // Nombre à virgule flottante 64 bits
```

### Booléens

Les valeurs booléennes représentent vrai ou faux :

```rust
let b: bool = true;  // Valeur booléenne
```

### Caractères

Les caractères représentent des valeurs scalaires Unicode uniques :

```rust
let c: char = 'λ';   // Caractère Unicode (lambda)
```

## 📦 Types Composés

Les types composés regroupent plusieurs valeurs en un seul type.

### Tuples

Les tuples sont des collections de taille fixe qui peuvent contenir des types mixtes :

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);  // Déclaration de tuple
let (a, b, c) = tup;                       // Déstructuration
println!("tuple: ({}, {}, {})", a, b, c);  // Accès aux éléments
```

### Tableaux

Les tableaux sont des collections de taille fixe d'éléments du même type :

```rust
let arr: [i32; 4] = [1, 2, 3, 4];  // Déclaration de tableau
println!("array: {:?}", arr);       // Imprimer le tableau entier
```

### Tranches

Les tranches fournissent une vue dans les tableaux sans prendre possession :

```rust
fn print_slice(slice: &[i32]) {
    println!("slice: {:?}", slice);
}
print_slice(&arr);  // Passer le tableau en tant que tranche
```

## 🦙 Propriété

La propriété est la fonctionnalité la plus unique de Rust et permet la sécurité mémoire sans collecteur d'ordures.

### Règles de Propriété

1. **Chaque valeur a une variable qui en est le propriétaire**
2. **Chaque valeur ne peut avoir qu'un seul propriétaire à la fois**
3. **Lorsque le propriétaire sort de la portée, la valeur est supprimée**

```rust
let s = String::from("hello");  // s possède le String
takes_ownership(s);             // s est déplacé vers la fonction
// s n'est plus valide ici
```

### Déplacement vs Copie

- **Déplacement** : Transfère la propriété (pour les données allouées sur le tas comme `String`)
- **Copie** : Crée un duplicata (pour les données uniquement sur la pile comme les entiers)

```rust
let x = 5;        // i32 implémente Copy
let y = x;        // x est copié, les deux sont valides
println!("x: {}, y: {}", x, y);  // Fonctionne bien

let s1 = String::from("hello");  // String n'implémente pas Copy
let s2 = s1;                     // s1 est déplacé vers s2
// println!("{}", s1);           // Cela ne compilerait pas
```

## 🦙 Références et Emprunt

Les références vous permettent d'accéder aux données sans prendre possession.

### Références Immuables

```rust
let s2 = String::from("world");
borrow_string(&s2);              // Emprunter s2 de manière immuable
println!("s2 after borrow: {}", s2);  // s2 est toujours valide
```

### Références Mutables

```rust
fn change_string(s: &mut String) {
    s.push_str(" world");
}

let mut s = String::from("hello");
change_string(&mut s);
println!("{}", s);  // Imprime "hello world"
```

### Règles de Référence

1. **Vous pouvez avoir soit une référence mutable SOIT plusieurs références immuables**
2. **Les références doivent toujours être valides**

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
cd a_02_data_types

# Exécuter avec Cargo (recommandé)
cargo run

# Ou compiler manuellement
cargo build
./target/debug/a_02_data_types.exe  # Sur Windows
# ou
./target/debug/a_02_data_types      # Sur Linux/macOS
```

### Sortie Attendue

```text
i32: -42 | u64: 42 | f64: 3.1415 | bool: true | char: λ
tuple: (500, 6.4, 1)
array: [1, 2, 3, 4]
slice: [1, 2, 3, 4]
I took ownership of: hello
I borrowed: world
s2 after borrow: world
```

## 🦙 Concepts Clés

### Gestion de la Mémoire

- **Pile** : Rapide, allocation/désallocation automatique, taille fixe
- **Tas** : Plus lent, allocation manuelle, taille variable
- **Propriété** : Assure la sécurité mémoire et empêche les fuites mémoire

### Avantages de l'Emprunt

- **Abstractions à coût zéro** : Les références n'ajoutent pas de surcharge à l'exécution
- **Empêche les courses de données** : Les règles d'emprunt mutable empêchent la modification concurrente
- **Flexible** : Peut emprunter de manière immuable plusieurs fois ou mutable une fois

### Modèles Courants

1. **Utilisez des références lorsque vous n'avez pas besoin de propriété**
2. **Utilisez `&mut` lorsque vous devez modifier les données empruntées**
3. **Retournez des valeurs possédées lorsque l'appelant doit prendre possession**
4. **Utilisez des tranches pour un accès flexible aux tableaux**

### Implications de Performance

- **Types de copie** (entiers, flottants, booléens, caractères) : Peu coûteux à passer
- **Types de déplacement** (String, Vec) : Transférez la propriété lorsque possible
- **Types d'emprunt** : Utilisez des références pour éviter les copies inutiles

## 🌚 Exercices

1. **Exploration des Types** : Expérimentez avec différents types scalaires et leurs plages
2. **Opérations sur les Tuples** : Créez des tuples avec différentes combinaisons de types et pratiquez la déstructuration
3. **Manipulation des Tableaux** : Créez des tableaux de différentes tailles et accédez aux éléments en utilisant l'indexation
4. **Transfert de Propriété** : Écrivez des fonctions qui prennent possession vs. empruntent des paramètres
5. **Pratique des Références** : Créez des fonctions qui acceptent des références mutables et immuables
6. **Opérations sur les Tranches** : Pratiquez la création de tranches à partir de tableaux et leur passage aux fonctions

## Lectures Supplémentaires

- [Livre Rust - Types de Données](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Livre Rust - Propriété](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Livre Rust - Références et Emprunt](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Référence Rust - Types](https://doc.rust-lang.org/reference/types.html)

## Notes

- **La propriété** est l'arme secrète de Rust pour la sécurité mémoire
- **L'emprunt** permet un code efficace sans sacrifier la sécurité
- **Les types** déterminent si les valeurs sont copiées ou déplacées
- Pratiquez ces concepts régulièrement - ils sont fondamentaux pour la programmation Rust
