# Wie ausführen

Gehen Sie zum Projektverzeichnis und führen Sie aus:

```bash
cargo run
```

## Verfügbare Sprachen

Diese Dokumentation ist auch in anderen Sprachen verfügbar:

- [Chinese](docs/README[Chinese].md)
- [Hindi](docs/README[French].md)
- [Spanish](docs/README[German].md)
- [Spanish](docs/README[Hindi].md)
- [Spanish](docs/README[Portuguese].md)
- [Spanish](docs/README[Russian].md)
- [Spanish](docs/README[Spanish].md)
- [English](../README.md)

## Rust Datentypen und Eigentümerschaft

Diese Lektion führt Rusts Datentypen und grundlegende Konzepte von Eigentümerschaft und Ausleihen ein. Das Verständnis dieser Konzepte ist entscheidend für das Schreiben sicherer und effizienter Rust-Code.

## Inhaltsverzeichnis

- [🦙 Skalare Typen](#-skalare-typen)
- [🦙 Zusammengesetzte Typen](#-zusammengesetzte-typen)
- [🦙 Eigentümerschaft](#-eigentümerschaft)
- [🦙 Referenzen und Ausleihen](#-referenzen-und-ausleihen)
- [🚀 Ausführen des Beispiels](#-ausführen-des-beispiels)
- [🦙 Schlüsselkonzepte](#-schlüsselkonzepte)
- [🦙 Übungen](#-übungen)

## 🦙 Skalare Typen

Skalare Typen repräsentieren einzelne Werte. Rust hat vier primäre skalare Typen: Ganzzahlen, Gleitkommazahlen, Boolesche Werte und Zeichen.

### Ganzzahlen

Rust bietet verschiedene Ganzzahltypen mit unterschiedlichen Größen und signierten/unsignierten Varianten:

```rust
let x: i32 = -42;    // Signierte 32-Bit-Ganzzahl
let y: u64 = 42;     // Unsignierte 64-Bit-Ganzzahl
```

### Gleitkommazahlen

Rust unterstützt zwei Gleitkommatypen:

```rust
let f: f64 = 3.1415; // 64-Bit-Gleitkommazahl
```

### Boolesche Werte

Boolesche Werte repräsentieren wahr oder falsch:

```rust
let b: bool = true;  // Boolescher Wert
```

### Zeichen

Zeichen repräsentieren einzelne Unicode-Skalarwerte:

```rust
let c: char = 'λ';   // Unicode-Zeichen (Lambda)
```

## 📦 Zusammengesetzte Typen

Zusammengesetzte Typen gruppieren mehrere Werte in einen Typ.

### Tupel

Tupel sind Sammlungen fester Größe, die gemischte Typen enthalten können:

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);  // Tupel-Deklaration
let (a, b, c) = tup;                       // Destrukturierung
println!("tuple: ({}, {}, {})", a, b, c);  // Zugriff auf Elemente
```

### Arrays

Arrays sind Sammlungen fester Größe von Elementen desselben Typs:

```rust
let arr: [i32; 4] = [1, 2, 3, 4];  // Array-Deklaration
println!("array: {:?}", arr);       // Gesamtes Array drucken
```

### Slices

Slices bieten eine Ansicht in Arrays, ohne Eigentümerschaft zu übernehmen:

```rust
fn print_slice(slice: &[i32]) {
    println!("slice: {:?}", slice);
}
print_slice(&arr);  // Array als Slice übergeben
```

## 🦙 Eigentümerschaft

Eigentümerschaft ist Rusts einzigartigstes Merkmal und ermöglicht Speichersicherheit ohne Garbage Collector.

### Eigentümerschaftsregeln

1. **Jeder Wert hat eine Variable, die sein Eigentümer ist**
2. **Jeder Wert kann nur einen Eigentümer gleichzeitig haben**
3. **Wenn der Eigentümer den Gültigkeitsbereich verlässt, wird der Wert gelöscht**

```rust
let s = String::from("hello");  // s besitzt den String
takes_ownership(s);             // s wird zur Funktion verschoben
// s ist hier nicht mehr gültig
```

### Verschieben vs Kopieren

- **Verschieben**: Überträgt Eigentümerschaft (für heap-zugewiesene Daten wie `String`)
- **Kopieren**: Erstellt ein Duplikat (für stack-only Daten wie Ganzzahlen)

```rust
let x = 5;        // i32 implementiert Copy
let y = x;        // x wird kopiert, beide sind gültig
println!("x: {}, y: {}", x, y);  // Funktioniert einwandfrei

let s1 = String::from("hello");  // String implementiert nicht Copy
let s2 = s1;                     // s1 wird zu s2 verschoben
// println!("{}", s1);           // Dies würde nicht kompilieren
```

## 🦙 Referenzen und Ausleihen

Referenzen ermöglichen Ihnen den Zugriff auf Daten, ohne Eigentümerschaft zu übernehmen.

### Unveränderliche Referenzen

```rust
let s2 = String::from("world");
borrow_string(&s2);              // s2 unveränderlich ausleihen
println!("s2 after borrow: {}", s2);  // s2 ist immer noch gültig
```

### Veränderliche Referenzen

```rust
fn change_string(s: &mut String) {
    s.push_str(" world");
}

let mut s = String::from("hello");
change_string(&mut s);
println!("{}", s);  // Druckt "hello world"
```

### Referenzregeln

1. **Sie können entweder eine veränderliche Referenz ODER mehrere unveränderliche Referenzen haben**
2. **Referenzen müssen immer gültig sein**

## 🚀 Ausführen des Beispiels

### Voraussetzungen

Stellen Sie sicher, dass Rust auf Ihrem System installiert ist.

```bash
# Überprüfen, ob Rust installiert ist
rustc --version
cargo --version
```

### Kompilierung und Ausführung

```bash
# Zum Projektverzeichnis navigieren
cd a_02_data_types

# Mit Cargo ausführen (empfohlen)
cargo run

# Oder manuell kompilieren
cargo build
./target/debug/a_02_data_types.exe  # Auf Windows
# oder
./target/debug/a_02_data_types      # Auf Linux/macOS
```

### Erwartete Ausgabe

```text
i32: -42 | u64: 42 | f64: 3.1415 | bool: true | char: λ
tuple: (500, 6.4, 1)
array: [1, 2, 3, 4]
slice: [1, 2, 3, 4]
I took ownership of: hello
I borrowed: world
s2 after borrow: world
```

## 🦙 Schlüsselkonzepte

### Speicherverwaltung

- **Stack**: Schnell, automatische Zuweisung/Freigabe, feste Größe
- **Heap**: Langsamer, manuelle Zuweisung, variable Größe
- **Eigentümerschaft**: Gewährleistet Speichersicherheit und verhindert Speicherlecks

### Vorteile der Ausleihe

- **Zero-Cost-Abstraktionen**: Referenzen fügen keine Laufzeit-Overhead hinzu
- **Verhindert Datenrennen**: Veränderliche Ausleihregeln verhindern gleichzeitige Modifikation
- **Flexibel**: Kann unveränderlich mehrmals oder veränderlich einmal ausleihen

### Häufige Muster

1. **Verwenden Sie Referenzen, wenn Sie keine Eigentümerschaft benötigen**
2. **Verwenden Sie `&mut`, wenn Sie ausgeliehene Daten modifizieren müssen**
3. **Geben Sie Eigentumswerte zurück, wenn der Aufrufer Eigentümerschaft übernehmen soll**
4. **Verwenden Sie Slices für flexiblen Array-Zugriff**

### Leistungsfolgen

- **Kopiertypen** (Ganzzahlen, Gleitkommazahlen, Boolesche Werte, Zeichen): Günstig zu übergeben
- **Verschiebungstypen** (String, Vec): Eigentümerschaft nach Möglichkeit übertragen
- **Ausleihtypen**: Referenzen verwenden, um unnötiges Kopieren zu vermeiden

## 🌚 Übungen

1. **Typ-Erkundung**: Experimentieren Sie mit verschiedenen skalaren Typen und ihren Bereichen
2. **Tupel-Operationen**: Erstellen Sie Tupel mit verschiedenen Typkombinationen und üben Sie die Destrukturierung
3. **Array-Manipulation**: Erstellen Sie Arrays unterschiedlicher Größen und greifen Sie mit Indizierung auf Elemente zu
4. **Eigentümerschaftsübertragung**: Schreiben Sie Funktionen, die Eigentümerschaft vs. Ausleihparameter übernehmen
5. **Referenz-Praxis**: Erstellen Sie Funktionen, die veränderliche und unveränderliche Referenzen akzeptieren
6. **Slice-Operationen**: Üben Sie das Erstellen von Slices aus Arrays und deren Übergabe an Funktionen

## Weiterführende Literatur

- [Rust-Buch - Datentypen](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Rust-Buch - Eigentümerschaft](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Rust-Buch - Referenzen und Ausleihen](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Rust-Referenz - Typen](https://doc.rust-lang.org/reference/types.html)

## Hinweise

- **Eigentümerschaft** ist Rusts Geheimwaffe für Speichersicherheit
- **Ausleihen** ermöglicht effizienten Code ohne Sicherheitsverlust
- **Typen** bestimmen, ob Werte kopiert oder verschoben werden
- Üben Sie diese Konzepte regelmäßig - sie sind grundlegend für die Rust-Programmierung
