# Wie ausführen

```bash
rustc 02_type_of_numbers.rs
./02_type_of_numbers
```

## Verfügbare Sprachen

Diese Dokumentation ist auch in anderen Sprachen verfügbar:

- [Chinese](docs/README[Chinese].md)
- [Hindi](docs/README[French].md)
- [Spanish](docs/README[Hindi].md)
- [Spanish](docs/README[Portuguese].md)
- [Spanish](docs/README[Russian].md)
- [Spanish](docs/README[Spanish].md)
- [English](../README.md)

# Rust Zahlentypen

Diese Lektion demonstriert die verschiedenen in Rust verfügbaren Zahlentypen, einschließlich Ganzzahlen und Gleitkommazahlen.

## 📋 Inhaltsverzeichnis

- [🔢 Ganzzahltypen](#-ganzzahltypen)
- [📊 Gleitkommatypen](#-gleitkommatypen)
- [🔤 Zahlenliterale](#-zahlenliterale)
- [🎯 Typinferenz](#-typinferenz)
- [➗ Arithmetische Operationen](#-arithmetische-operationen)
- [🚀 Ausführen des Beispiels](#-ausführen-des-beispiels)

## 🔢 Ganzzahltypen

Rust bietet mehrere Ganzzahltypen mit unterschiedlichen Größen und Vorzeichen-/Vorzeichenlosen-Varianten.

### Vorzeichenlose Ganzzahlen

Vorzeichenlose Ganzzahlen können nur positive Zahlen (einschließlich Null) darstellen.

| Typ | Größe | Bereich | Beispiel |
|-----|-------|---------|----------|
| `u8` | 8 Bits | 0 bis 255 | `let x: u8 = 255;` |
| `u16` | 16 Bits | 0 bis 65.535 | `let x: u16 = 65_535;` |
| `u32` | 32 Bits | 0 bis 4.294.967.295 | `let x: u32 = 4_294_967_295;` |
| `u64` | 64 Bits | 0 bis 18.446.744.073.709.551.615 | `let x: u64 = 18_446_744_073_709_551_615;` |
| `u128` | 128 Bits | 0 bis 340.282.366.920.938.463.463.374.607.431.768.211.455 | `let x: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;` |
| `usize` | Architekturabhängig | 32-Bit: 0 bis 4.294.967.295 / 64-Bit: 0 bis 18.446.744.073.709.551.615 | `let x: usize = 100;` |

### Vorzeichenbehaftete Ganzzahlen

Vorzeichenbehaftete Ganzzahlen können sowohl positive als auch negative Zahlen darstellen.

| Typ | Größe | Bereich | Beispiel |
|-----|-------|---------|----------|
| `i8` | 8 Bits | -128 bis 127 | `let x: i8 = -128;` |
| `i16` | 16 Bits | -32.768 bis 32.767 | `let x: i16 = 32_767;` |
| `i32` | 32 Bits | -2.147.483.648 bis 2.147.483.647 | `let x: i32 = -2_147_483_648;` |
| `i64` | 64 Bits | -9.223.372.036.854.775.808 bis 9.223.372.036.854.775.807 | `let x: i64 = 9_223_372_036_854_775_807;` |
| `i128` | 128 Bits | -170.141.183.460.469.231.731.687.303.715.884.105.728 bis 170.141.183.460.469.231.731.687.303.715.884.105.727 | `let x: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728;` |
| `isize` | Architekturabhängig | 32-Bit: -2.147.483.648 bis 2.147.483.647 / 64-Bit: -9.223.372.036.854.775.808 bis 9.223.372.036.854.775.807 | `let x: isize = -50;` |

## 📊 Gleitkommatypen

Gleitkommatypen stellen Dezimalzahlen mit unterschiedlichen Präzisionsstufen dar.

| Typ | Größe | Präzision | Beispiel |
|-----|-------|-----------|----------|
| `f32` | 32 Bits | ~6-9 Dezimalstellen | `let x: f32 = 3.141592;` |
| `f64` | 64 Bits | ~15-17 Dezimalstellen | `let x: f64 = 2.718281828459045;` |

**Hinweis**: `f64` ist der Standard-Gleitkommatyp in Rust und wird aufgrund seiner höheren Präzision allgemein bevorzugt.

## 🔤 Zahlenliterale

Rust unterstützt verschiedene Möglichkeiten, Zahlenliterale für bessere Lesbarkeit zu schreiben.

### Unterstrich-Trennzeichen

```rust
let million = 1_000_000;        // Gleich wie 1000000
let binaer = 0b_1111_0000;      // Binär mit Trennzeichen
let hex = 0x_DEAD_BEEF;         // Hexadezimal mit Trennzeichen
```

### Verschiedene Basen

```rust
let dezimal = 42;               // Dezimal (Standard)
let hex = 0x2A;                 // Hexadezimal (0x-Präfix)
let oktal = 0o52;               // Oktal (0o-Präfix)
let binaer = 0b101010;          // Binär (0b-Präfix)
```

### Typsuffixe

```rust
let x = 42u32;                  // u32-Suffix
let y = 3.14f32;                // f32-Suffix
let z = 1_000_000i64;           // i64-Suffix
```

## 🎯 Typinferenz

Rust kann oft den Typ einer Variablen aus ihrem Kontext ableiten.

```rust
let x = 42;         // i32 (Standard-Ganzzahltyp)
let y = 3.14;       // f64 (Standard-Gleitkommatyp)
let z = 42u8;       // u8 (explizite Typannotation)
```

## ➗ Arithmetische Operationen

Alle Zahlentypen unterstützen grundlegende arithmetische Operationen.

```rust
let a = 10;
let b = 3;

let summe = a + b;        // Addition
let differenz = a - b;    // Subtraktion
let produkt = a * b;      // Multiplikation
let quotient = a / b;     // Division
let rest = a % b;         // Modulo
```

### Überlaufverhalten

- **Debug-Modus**: Programm panics bei Ganzzahlüberlauf
- **Release-Modus**: Wrappt um (Zweierkomplement)

```rust
let x: u8 = 255;
let y = x + 1;  // Panics im Debug, wrappt zu 0 im Release
```

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
cd b_02_type_of_numbers

# Programm kompilieren
rustc 02_type_of_numbers.rs

# Ausführbare Datei ausführen
./02_type_of_numbers.exe  # Unter Windows
# oder
./02_type_of_numbers      # Unter Linux/macOS
```

### Erwartete Ausgabe

```text
Vorzeichenlose Ganzzahlen: u8=255 u16=65535 u32=4294967295 u64=18446744073709551615 u128=340282366920938463463374607431768211455 usize=100
Vorzeichenbehaftete Ganzzahlen: i8=-128 i16=32767 i32=-2147483648 i64=9223372036854775807 i128=-170141183460469231731687303715884105728 isize=-50
Gleitkommazahlen: f32=3.141592 f64=2.718281828459045
Dies wird in der gleichen Zeile ausgegeben!
```

## 📚 Schlüsselkonzepte

### Speichernutzung

- `u8` und `i8`: 1 Byte
- `u16` und `i16`: 2 Bytes
- `u32` und `i32`: 4 Bytes
- `u64` und `i64`: 8 Bytes
- `u128` und `i128`: 16 Bytes
- `f32`: 4 Bytes
- `f64`: 8 Bytes

### Leistungsüberlegungen

- Kleinere Typen verwenden weniger Speicher und können schneller sein
- `f64` wird `f32` für die meisten Berechnungen vorgezogen
- `i32` ist oft die beste Wahl für Ganzzahlen für allgemeine Zwecke
- Verwenden Sie `usize`/`isize` bei der Arbeit mit Speicherindizes

### Häufige Fallstricke

1. **Ganzzahlüberlauf**: Seien Sie vorsichtig mit arithmetischen Operationen, die die Typgrenzen überschreiten könnten
2. **Typinkompatibilitäten**: Operationen zwischen verschiedenen Typen ohne explizite Umwandlung sind nicht möglich
3. **Präzisionsverlust**: `f32` hat weniger Präzision als `f64`
4. **Plattformunterschiede**: Die Größe von `usize`/`isize` variiert zwischen 32-Bit- und 64-Bit-Systemen

## 🔍 Weiterführende Literatur

- [Rust-Buch - Datentypen](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Rust-Referenz - Typen](https://doc.rust-lang.org/reference/types.html)
- [Dokumentation der Primitiven Typen](https://doc.rust-lang.org/std/index.html#primitives)

## 🧪 Übungen

1. **Typerkundung**: Versuchen Sie, die Werte im Beispiel zu ändern, um zu sehen, was mit verschiedenen Bereichen passiert
2. **Arithmetische Operationen**: Fügen Sie Code hinzu, um verschiedene arithmetische Operationen durchzuführen und Ergebnisse auszugeben
3. **Typumwandlung**: Experimentieren Sie mit der Umwandlung zwischen verschiedenen Zahlentypen
4. **Überlauf-Tests**: Testen Sie, was passiert, wenn Sie die Grenzen verschiedener Typen überschreiten

## 📝 Hinweise

- Dieses Beispiel demonstriert die maximalen und minimalen Werte für jeden Typ
- In der Praxis werden Sie oft `i32` für Ganzzahlen und `f64` für Gleitkommazahlen verwenden
- Das Typsystem von Rust hilft, viele häufige Programmierfehler zu verhindern
- Berücksichtigen Sie immer den Wertebereich, den Ihr Programm benötigt, bei der Auswahl von Typen

---

## 🌍 Verfügbare Sprachen

Diese Dokumentation ist auch in anderen Sprachen verfügbar:

- [Spanish](README[Spanish].md)
- [Hindi](README[Hindi].md)
- [Chinese](README[Chinese].md)

## 📚 Kursinformationen

Diese Lektion ist Teil des Rust-Lernkurses
