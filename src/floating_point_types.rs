// Module: Floating-Point Types
// This module introduces Rust's floating-point number types.

pub fn demonstrate_floating_point_types() {
    println!("\n--- Floating-Point Types ---");

    // Rust has two primary floating-point types: f32 and f64.
    // f32 is a single-precision float.
    // f64 is a double-precision float.
    // The default type is f64 because on modern CPUs, it's roughly the same speed as f32
    // but is capable of more precision.

    let x: f32 = 3.14; // Explicitly an f32
    let y = 2.71828;   // Implicitly an f64 (default type for floating-point literals)
    let z: f64 = 1.618; // Explicitly an f64

    println!("x (f32): {}", x);
    println!("y (f64 default): {}", y);
    println!("z (f64 explicit): {}", z);

    // Floating-point numbers can be used in arithmetic operations.
    let sum_float = x + y as f32; // y (f64) needs to be cast to f32 for this operation
    println!("Sum of x + (y as f32): {}", sum_float);

    let product_float = y * z; // Both are f64, so no cast needed
    println!("Product of y * z: {}", product_float);

    // Floating point numbers are not always exact and can lead to precision issues.
    let a: f32 = 0.1;
    let b: f32 = 0.2;
    let sum_ab: f32 = a + b;
    println!("0.1 (f32) + 0.2 (f32) = {} (watch for precision!)", sum_ab); // Might not be exactly 0.3

    // For applications requiring high precision without rounding errors (like financial calculations),
    // consider using specialized crates like `rust_decimal`.

    // Original prac6.rs:
    // fn main(){
    //     println!("Floating point numbers in Rust");
    //     let x:f32 = 3.14;
    //     let y = 2.1346; //by default, it is f64
    //     println!("x = {}, y = {}", x, y);
    // }
    // This refactored module expands on these concepts.
}
