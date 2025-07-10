// Module: References and Borrowing
// This module explains how references allow you to use values without taking ownership.

pub fn demonstrate_references_and_borrowing() {
    println!("\n--- References and Borrowing ---");

    let s1: String = String::from("Hello");
    println!("s1 initially: \"{}\"", s1);

    // Pass an immutable reference of s1 to calculate_length.
    // s1 is "borrowed" by calculate_length but ownership remains with demonstrate_references_and_borrowing.
    let len: usize = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len); // s1 can still be used here.

    // Pass a mutable reference of s2 to change_string.
    let mut s2: String = String::from("Mutable");
    println!("s2 before change: \"{}\"", s2);
    change_string(&mut s2);
    println!("s2 after change: \"{}\"", s2);

    // Rules of References:
    // 1. At any given time, you can have either one mutable reference or any number of immutable references.
    // 2. References must always be valid (no dangling references).

    // Example of multiple immutable references:
    let r1 = &s2;
    let r2 = &s2;
    println!("Immutable references r1: \"{}\", r2: \"{}\"", r1, r2);
    // We can have multiple immutable borrows. No problem here.

    // Example of one mutable reference:
    let mut s3 = String::from("Another string");
    let r3 = &mut s3;
    r3.push_str(" modified");
    println!("Mutable reference r3 (now s3): \"{}\"", r3); // or s3

    // The following would cause a compile error if uncommented because you cannot have a mutable reference
    // while immutable references exist, or multiple mutable references to the same data simultaneously.
    // let mut s_conflict = String::from("conflict");
    // let r_immut = &s_conflict;
    // let r_mut = &mut s_conflict; // ERROR: cannot borrow `s_conflict` as mutable because it is also borrowed as immutable
    // println!("{}, {}", r_immut, r_mut);

    // Or:
    // let mut s_mut_conflict = String::from("mut_conflict");
    // let r_mut1 = &mut s_mut_conflict;
    // let r_mut2 = &mut s_mut_conflict; // ERROR: cannot borrow `s_mut_conflict` as mutable more than once at a time
    // println!("{}, {}", r_mut1, r_mut2);
    println!("(Examples of conflicting borrows are commented out to allow compilation)");
}

// This function takes an immutable reference to a String.
// It "borrows" the String but does not take ownership.
fn calculate_length(s: &String) -> usize {
    // s.push_str(" - try to change"); // This would be an error, cannot modify a borrowed immutable String.
    s.len()
} // s goes out of scope here. Since it's a reference, the String it refers to is not dropped.

// This function takes a mutable reference to a String.
// It can modify the borrowed String.
fn change_string(some_string: &mut String) {
    some_string.push_str(" world!");
} // some_string goes out of scope.

// Original prac3.rs:
// fn main(){
//     let mut var1:String = String::from("Hello");
//     modified_string(&mut var1);
//     let var2:usize = get_length(&var1);
//     println!("length of {} is {} " , var1, var2);
// }
// fn get_length(a: &String) -> usize{
//     let length: usize = a.len();
//     return length;
// }
// fn modified_string(b: &mut String){
//     b.push_str(" Blockchain!");
// }
// The refactored code demonstrates these concepts.
