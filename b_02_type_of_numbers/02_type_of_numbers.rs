fn main() {
    // ============================
    // TIPOS ENTEROS EN RUST
    // ============================

    // Enteros sin signo (solo positivos)
    let a: u8 = 255;      // 8 bits (0 a 255)
    let b: u16 = 65_535;  // 16 bits (0 a 65,535)
    let c: u32 = 4_294_967_295; // 32 bits
    let d: u64 = 18_446_744_073_709_551_615; // 64 bits
    let e: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455; // 128 bits
    let f: usize = 100;   // Depende de la arquitectura (32 o 64 bits)

    // Enteros con signo (positivos y negativos)
    let g: i8 = -128;     // 8 bits (-128 a 127)
    let h: i16 = 32_767;  // 16 bits
    let i: i32 = -2_147_483_648; // 32 bits
    let j: i64 = 9_223_372_036_854_775_807; // 64 bits
    let k: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728; // 128 bits
    let l: isize = -50;   // Depende de la arquitectura (32 o 64 bits)

    // ============================
    // TIPOS DE PUNTO FLOTANTE
    // ============================

    let x: f32 = 3.141592;  // Número decimal de 32 bits (menos precisión)
    let y: f64 = 2.718281828459045; // Número decimal de 64 bits (por defecto en Rust)

    // ============================
    // EJEMPLOS DE USO
    // ============================

    println!("Enteros sin signo: u8={} u16={} u32={} u64={} u128={} usize={}", a, b, c, d, e, f);
    println!("Enteros con signo: i8={} i16={} i32={} i64={} i128={} isize={}", g, h, i, j, k, l);
    println!("Decimales: f32={} f64={}", x, y);
}
