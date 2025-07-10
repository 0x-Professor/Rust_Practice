// Module: Boolean Type
// This module covers the boolean type in Rust.

pub fn demonstrate_boolean_type() {
    println!("\n--- Boolean Type ---");

    // Rust's boolean type is `bool`, which can have two possible values: `true` or `false`.
    let is_sunny: bool = true;
    let is_raining = false; // Type `bool` is inferred here

    println!("Is it sunny? {}", is_sunny);
    println!("Is it raining? {}", is_raining);

    // Booleans are commonly used in control flow statements (like if/else).
    if is_sunny {
        println!("It's a sunny day! Don't forget your sunglasses.");
    } else {
        println!("It's not sunny today.");
    }

    if is_raining {
        println!("It's raining! Remember your umbrella.");
    } else {
        println!("No rain for now.");
    }

    // Boolean logic operators:
    // `&&` (logical AND)
    // `||` (logical OR)
    // `!`  (logical NOT)

    let have_sunglasses = true;
    let have_umbrella = false;

    let go_to_beach = is_sunny && !is_raining;
    println!("Should we go to the beach? {}", go_to_beach);

    let stay_inside = is_raining || !is_sunny;
    println!("Should we stay inside? {}", stay_inside);

    let need_sunglasses_today = is_sunny && have_sunglasses;
    let need_umbrella_today = is_raining && have_umbrella; // This was `is_sunny && is_raining` in prac7, which is less logical

    println!("Do I need sunglasses today? {}", need_sunglasses_today);
    println!("Do I need an umbrella today? {}", need_umbrella_today); // Corrected logic from prac7

    // Original prac7.rs:
    // fn main(){
    //     let is_sunny:bool = true;
    //     let is_raining = false; // by default, it is bool
    //     let need_umbrella = is_sunny && is_raining; // This logic was a bit off for "need_umbrella"
    //     let need_sunglasses = is_sunny || is_raining; // This logic was also a bit off
    //     println!("need_umbrella: {}", need_umbrella);
    //     println!("need_sunglasses: {}", need_sunglasses);
    // }
    // The refactored module provides clearer examples and slightly adjusted logic for clarity.
}
