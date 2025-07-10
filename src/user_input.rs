// Module: User Input
// This module demonstrates how to read basic user input from the console.

// Import the `io` (input/output) module from the standard library.
use std::io;

pub fn demonstrate_user_input() {
    println!("\n--- User Input ---");

    // Note: For a real application, robust error handling and input validation are crucial.
    // This example keeps it simple to demonstrate the core mechanism.

    println!("Please enter your name:");

    // Create a mutable String to store the user's input.
    // `String::new()` creates an empty string.
    let mut name_input = String::new();

    // `io::stdin()` returns a handle to the standard input stream of the current process.
    // `.read_line(&mut name_input)` reads a line of text from standard input
    // and appends it to the `name_input` string.
    // It returns an `io::Result<usize>`, which is an enum that can be `Ok(number_of_bytes_read)`
    // or `Err(error_details)`.
    // `.expect("Failed to read line")` is a shortcut to handle the Result.
    // If it's an `Err`, the program will panic and display the provided message.
    // If it's `Ok`, it will unwrap the value (number of bytes read) and proceed.
    io::stdin()
        .read_line(&mut name_input)
        .expect("Failed to read line. Please try again.");

    // `read_line` includes the newline character (`\n`) when the user presses Enter.
    // We often want to remove this. `trim()` removes leading and trailing whitespace.
    let name = name_input.trim();

    println!("Hello, {}! Nice to meet you.", name);

    // Reading numbers requires parsing the String input.
    println!("\nPlease enter your age:");
    let mut age_input = String::new();

    io::stdin()
        .read_line(&mut age_input)
        .expect("Failed to read age. Please enter a valid number.");

    // `trim()` the input, then `parse()` it into the desired numeric type (e.g., u32).
    // `parse()` returns a `Result` because the input might not be a valid number.
    match age_input.trim().parse::<u32>() {
        Ok(age) => {
            println!("You are {} years old.", age);
            if age >= 18 {
                println!("You are an adult.");
            } else {
                println!("You are a minor.");
            }
        }
        Err(parse_error) => {
            // If parsing fails, print an error message.
            println!("Invalid age entered. That doesn't look like a number. Error: {}", parse_error);
        }
    }

    // To make this module runnable without actual user input during automated tests or
    // if the user just wants to see the output without typing, we could add a non-interactive
    // part or use conditional compilation, but for this tutorial, direct input is shown.
    // For the `cargo run` in this tutorial, the user will be prompted.

    // Original prac14.rs:
    // use std::io;
    // fn main(){
    //     let mut input = String::new();
    //     println!("Please Enter your name:");
    //     io::stdin().read_line(&mut input).expect("Failed to read line");
    //     println!("Hello, !{}", input); // Note: original had an extra '!' and didn't trim.
    // }
    // This refactored module provides a more complete example including number parsing.
}
