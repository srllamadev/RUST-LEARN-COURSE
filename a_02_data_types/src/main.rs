fn main() {
    // Scalar types
    let x: i32 = -42;
    let y: u64 = 42;
    let f: f64 = 3.1415;
    let b: bool = true;
    let c: char = 'λ';

    println!("i32: {} | u64: {} | f64: {} | bool: {} | char: {}", x, y, f, b, c);

    // Compound types
    // Tuple: fixed-size, can contain mixed types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup; // destructuring
    println!("tuple: ({}, {}, {})", a, b, c);

    // Array: fixed-size, all elements same type
    let arr: [i32; 4] = [1, 2, 3, 4];
    println!("array: {:?}", arr);

    // Slices (view into arrays) - demonstrated via function
    print_slice(&arr);

    // Ownership and references (brief)
    let s = String::from("hello");
    takes_ownership(s);
    // s is moved; the following line would not compile if uncommented:
    // println!("s after move: {}", s);

    let s2 = String::from("world");
    borrow_string(&s2);
    println!("s2 after borrow: {}", s2); // works because s2 was only borrowed
}

fn print_slice(slice: &[i32]) {
    println!("slice: {:?}", slice);
}

fn takes_ownership(s: String) {
    println!("I took ownership of: {}", s);
}

fn borrow_string(s: &String) {
    println!("I borrowed: {}", s);
}
