// Module: String Ownership
// This module demonstrates Rust's ownership system using the String type.

pub fn demonstrate_string_ownership() {
    println!("\n--- String Ownership ---");

    // s1 owns the String data "Hello, Blockchain!"
    let s1: String = String::from("Hello, Blockchain!");
    println!("s1 initially: \"{}\"", s1);

    // When s1 is passed to takes_ownership, ownership of the String data is moved.
    // s1 is no longer valid after this call.
    takes_ownership(s1);

    // Trying to use s1 here would cause a compile-time error because its value has been moved.
    // println!("Trying to use s1 after move: {}", s1); // This line would not compile!
    println!("s1 is no longer valid here (ownership moved to takes_ownership function).");

    // Integer types (and other types that implement the Copy trait) are copied, not moved.
    let x: i32 = 5;
    makes_copy(x);
    println!("x after calling makes_copy: {} (still valid as i32 has Copy trait)", x); // x is still valid

    // Returning ownership
    let s2: String = String::from("Return me");
    println!("s2 initially: \"{}\"", s2);
    let s3: String = takes_and_gives_back(s2);
    // s2 is no longer valid here, but s3 now owns the String data.
    println!("s3 (received from takes_and_gives_back): \"{}\"", s3);
    // println!("s2 after move to takes_and_gives_back: {}", s2); // This would not compile!
}

// This function takes ownership of the String passed to it.
fn takes_ownership(some_string: String) {
    println!("Inside takes_ownership: \"{}\"", some_string);
    // some_string goes out of scope here, and the String data is dropped (memory freed),
    // unless ownership was transferred out (e.g., by returning it).
}

// This function takes an i32, which implements the Copy trait.
// So, a copy of the value is made.
fn makes_copy(some_integer: i32) {
    println!("Inside makes_copy: {}", some_integer);
    // some_integer goes out of scope, but since it's a copy, the original is unaffected.
}

// This function takes ownership of a String and returns ownership of a String.
fn takes_and_gives_back(a_string: String) -> String {
    println!("Inside takes_and_gives_back, received: \"{}\"", a_string);
    // We are returning the String, so ownership is transferred out of this function.
    a_string
}

// Note: The original prac2.rs had a commented-out line:
// fn main(){
//     let string_var:String = String::from("Hello, Blockchain!");
//     print_value(string_var);
//     //println!("{}",string_var); // This was the important line demonstrating the move
// }
// fn print_value(string_var1:String){
//     println!("{}",string_var1);
// }
// The refactored code above captures this concept with s1 and takes_ownership.
