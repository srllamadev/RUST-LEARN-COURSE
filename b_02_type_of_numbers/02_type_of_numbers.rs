fn main() {
    // ============================
    // INTEGER TYPES
    // ============================

    // Unsigned integers (only positive numbers)
    let a: u8 = 255;      // 8-bit (0 to 255)
    let b: u16 = 65_535;  // 16-bit
    let c: u32 = 4_294_967_295; // 32-bit
    let d: u64 = 18_446_744_073_709_551_615; // 64-bit
    let e: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455; // 128-bit
    let f: usize = 100;   // Depends on the architecture (32 or 64 bits)

    // Signed integers (positive and negative numbers)
    let g: i8 = -128;     // 8-bit (-128 to 127)
    let h: i16 = 32_767;  // 16-bit
    let i: i32 = -2_147_483_648; // 32-bit
    let j: i64 = 9_223_372_036_854_775_807; // 64-bit
    let k: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728; // 128-bit
    let l: isize = -50;   // Depends on the architecture (32 or 64 bits)

    // ============================
    // FLOATING-POINT TYPES
    // ============================

    let x: f32 = 3.141592;  // 32-bit decimal (less precision)
    let y: f64 = 2.718281828459045; // 64-bit decimal (default in Rust, more precision)

    // ============================
    // PRINTING
    // ============================

    // println! -> prints with a newline at the end
    println!("Unsigned integers: u8={} u16={} u32={} u64={} u128={} usize={}", a, b, c, d, e, f);
    println!("Signed integers: i8={} i16={} i32={} i64={} i128={} isize={}", g, h, i, j, k, l);
    println!("Floating numbers: f32={} f64={}", x, y);

    // print! -> prints without newline
    print!("This is printed ");
    print!("on the same line!");
}
