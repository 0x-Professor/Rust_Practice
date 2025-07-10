// Module: Character Type
// This module explains Rust's character type `char`.

pub fn demonstrate_char_type() {
    println!("\n--- Character Type ---");

    // Rust's `char` type represents a single Unicode scalar value.
    // This means it can store much more than just ASCII characters.
    // Character literals are specified with single quotes.
    let c1: char = 'A';
    let c2: char = 'z';
    let c3: char = '7';
    let c4: char = '😻'; // Emoji character
    let c5: char = 'Ω'; // Greek letter Omega

    println!("c1 (letter): {}", c1);
    println!("c2 (letter): {}", c2);
    println!("c3 (digit): {}", c3);
    println!("c4 (emoji): {}", c4);
    println!("c5 (Greek letter): {}", c5);

    // `char` type is 4 bytes in size (UTF-32).
    println!("Size of char: {} bytes", std::mem::size_of::<char>());

    // A String is a collection of UTF-8 encoded characters.
    // While related, `char` and `String` are different.
    // A `String` is a growable, heap-allocated data structure, whereas `char` is a single character.
    let string_example: String = String::from("Hello, Rustaceans! 🦀");
    println!("Example String: {}", string_example);
    println!("Iterating through characters of the string:");
    for character in string_example.chars() {
        print!("'{}' ", character);
    }
    println!(); // Newline after loop

    // You can check properties of characters:
    println!("Is 'A' an alphabetic character? {}", 'A'.is_alphabetic());
    println!("Is '7' a numeric digit? {}", '7'.is_numeric());
    println!("Is '😻' an emoji (alphanumeric check)? {}", '😻'.is_alphanumeric()); // Emojis are often not alphanumeric
    println!("Is ' ' a whitespace character? {}", ' '.is_whitespace());

    // Original prac8.rs:
    // fn main(){
    //     let char_var: char = 'A';
    //     let emoji_var = '😊';
    //     let string_var: String = String::from("Hello, Blockchain!");
    //     println!("{} {} {}", char_var, emoji_var, string_var);
    // }
    // This refactored module provides more detail on the char type.
}
