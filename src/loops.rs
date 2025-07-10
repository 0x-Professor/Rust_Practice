// Module: Loops
// This module explains different types of loops in Rust: `loop`, `while`, and `for`.

pub fn demonstrate_loops() {
    println!("\n--- Loops: loop, while, for ---");

    // --- `loop` expression ---
    // The `loop` keyword creates an infinite loop.
    // You must use `break` to exit a `loop`, or it will run forever.
    // `loop` can also be used as an expression to return a value from a broken loop.
    println!("\nDemonstrating `loop` with break and returning a value:");
    let mut counter_loop = 0;
    let result_from_loop = loop {
        counter_loop += 1;
        print!("."); // Print a dot for each iteration to show progress
        if counter_loop == 5 {
            println!(" Counter in loop reached 5, breaking and returning value.");
            break counter_loop * 2; // Exit loop and return counter_loop * 2
        }
    };
    println!("Result returned from loop: {}", result_from_loop); // Should be 10

    // --- `while` loop ---
    // A `while` loop executes as long as a condition remains true.
    println!("\nDemonstrating `while` loop:");
    let mut counter_while = 0;
    while counter_while < 5 {
        print!("{} ", counter_while);
        counter_while += 1;
    }
    println!("\nCounter after while loop: {}", counter_while); // Should be 5

    // --- `for` loop ---
    // A `for` loop is used to iterate over a collection or a range.
    // It's generally safer and more concise than `while` loops for iteration
    // because it handles the iteration variable's state internally.

    // Iterating over a range
    println!("\nDemonstrating `for` loop with a range (1..5):"); // 1 up to (but not including) 5
    for i in 1..5 { // The range `1..5` creates numbers 1, 2, 3, 4
        print!("{} ", i);
    }
    println!();

    println!("Demonstrating `for` loop with an inclusive range (1..=5):"); // 1 up to (and including) 5
    for i in 1..=5 { // The range `1..=5` creates numbers 1, 2, 3, 4, 5
        print!("{} ", i);
    }
    println!();

    // Iterating over an array (or vector)
    let arr_for_loop: [&str; 5] = ["Hello", "Blockchain", "Rust", "Programming", "Language"];
    println!("\nDemonstrating `for` loop iterating over an array:");
    for element in arr_for_loop.iter() { // `.iter()` provides immutable references to elements
        print!("[{}] ", element);
    }
    println!();

    // Using `enumerate()` to get both index and value
    println!("\nDemonstrating `for` loop with `enumerate()`:");
    for (index, element) in arr_for_loop.iter().enumerate() {
        println!("Index: {}, Value: {}", index, element);
    }

    // Original prac13.rs functions:
    // fn main(){
    //     let mut counter:u8 = 0;
    //     loop{
    //         counter+=1;
    //         if counter == 5 {
    //             println!("Counter reached 5, exiting loop.");
    //             break;
    //         }
    //     }
    //     func_while();
    //     func_for();
    // }
    // fn func_while(){
    //     let mut counter:u8 =0;
    //     while counter !=5{
    //         counter +=1;
    //         println!("Counter in while loop: {}", counter);
    //     }
    // }
    // fn func_for(){
    //     for i in 1..5{
    //         println!("Counter in for loop: {}", i);
    //     }
    //     let arr:[&str;5] = ["Hello", "Blockchain", "Rust", "Programming", "Language"];
    //     for element in arr.iter(){
    //         println!("Array element: {}", element);
    //     }
    // }
    // The refactored module consolidates and expands these examples.
}
