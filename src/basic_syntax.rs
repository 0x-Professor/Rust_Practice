// Module: Basic Syntax, Variables, Functions, and Shadowing
// This module demonstrates basic Rust syntax, variable declaration (including mutability and shadowing),
// and function calls.

pub fn demonstrate_basics() {
    println!("\n--- Basic Syntax, Variables, Functions, and Shadowing ---");

    // --- Variables and Mutability ---
    // Variables are immutable by default.
    let immutable_var: u8 = 5;
    println!("Immutable variable 'immutable_var': {}", immutable_var);
    // immutable_var = 10; // This would cause a compile error because immutable_var is not mutable.

    // To make a variable mutable, use the `mut` keyword.
    let mut mutable_var: i32 = 10;
    println!("Initial value of 'mutable_var': {}", mutable_var);
    mutable_var = 20;
    println!("New value of 'mutable_var': {}", mutable_var);

    // --- Basic Data Types (Implicit and Explicit) ---
    let an_integer = 15; // Type i32 inferred by default for integers
    let a_float = 3.14;  // Type f64 inferred by default for floats
    let a_boolean = true; // Type bool inferred
    println!("Inferred types: integer={}, float={}, boolean={}", an_integer, a_float, a_boolean);

    // --- Function Calls ---
    // Call a simple function that prints a value.
    // Primitive types like u8 implement the `Copy` trait, so `an_integer_param` is copied.
    let an_integer_param: u8 = 25;
    print_value_passed(an_integer_param);
    println!("'an_integer_param' is still valid here: {}", an_integer_param);

    // Call a function that returns a value.
    let sum_result = add_numbers(10, 20);
    println!("The sum of 10 and 20 (from add_numbers function) is: {}", sum_result);

    // --- Shadowing ---
    // You can declare a new variable with the same name as a previous variable.
    // This is called "shadowing". The new variable "shadows" the previous one.
    // The previous variable is still there but inaccessible by its name in this scope.

    let shadow_example_num: i32 = 5;
    println!("'shadow_example_num' initially (integer): {}", shadow_example_num);

    // Shadowing `shadow_example_num` with a new variable of the same name but potentially different value.
    let shadow_example_num = shadow_example_num + 5;
    println!("'shadow_example_num' after shadowing (integer + 5): {}", shadow_example_num);

    // Shadowing is different from marking a variable as `mut` because:
    // 1. We are effectively creating a new variable.
    // 2. We can change the type of the variable when shadowing.
    // 3. It's useful if you want to perform some transformations on a value but keep it immutable afterwards.

    let spaces = "   "; // `spaces` is a string slice (&str)
    println!("'spaces' initially (string slice): \"{}\"", spaces);

    let spaces = spaces.len(); // `spaces` is shadowed by a new variable, now of type usize (length of the string)
    println!("'spaces' after shadowing (now length, type usize): {}", spaces);
    // This is allowed because the first `spaces` variable (the string slice) goes out of scope
    // (or rather, its name is reused), and a new `spaces` variable (the usize) is created.

    // Another example from original prac11:
    let x_shadow = 5;
    println!("x_shadow (original): {}", x_shadow); // x_shadow is i32

    let x_shadow = "Hello, Blockchain!"; // x_shadow is now &str
    println!("x_shadow (shadowed as &str): \"{}\"", x_shadow);

    let x_shadow = x_shadow.len(); // x_shadow is now usize
    println!("x_shadow (shadowed as usize - length of string): {}", x_shadow);
}

// This function takes a u8 value as an argument and prints it.
// 'value' is a copy of the argument passed to the function because u8 is a simple type
// that implements the Copy trait.
fn print_value_passed(value: u8) {
    println!("Value received in print_value_passed function: {}", value);
}

// Example of a function that returns a value.
// Public so it could potentially be used by other modules if this file were part of a larger library structure.
pub fn add_numbers(a: i32, b: i32) -> i32 {
    // In Rust, the last expression in a function is implicitly returned if there's no semicolon.
    // So, 'return a + b;' and 'a + b' are equivalent here.
    a + b
}

// --- Unit Tests ---
// Unit tests are typically placed in a sub-module named `tests` within the same file.
// The `#[cfg(test)]` attribute tells Rust to only compile and run this module
// when `cargo test` is executed.
#[cfg(test)]
mod tests {
    // `use super::*;` imports all items from the parent module (in this case, `basic_syntax`).
    use super::*;

    #[test]
    fn test_add_numbers_positive() {
        assert_eq!(add_numbers(2, 3), 5);
    }

    #[test]
    fn test_add_numbers_negative() {
        assert_eq!(add_numbers(-5, -3), -8);
    }

    #[test]
    fn test_add_numbers_mixed() {
        assert_eq!(add_numbers(5, -3), 2);
    }

    #[test]
    fn test_add_numbers_zero() {
        assert_eq!(add_numbers(0, 0), 0);
        assert_eq!(add_numbers(5, 0), 5);
        assert_eq!(add_numbers(0, -3), -3);
    }

    // Example of a test that uses an internal (non-pub) function if it existed.
    // Here, we'll test `print_value_passed` indirectly by checking compilation,
    // but typically you'd test functions that return values or have side effects
    // you can observe. Since `print_value_passed` only prints, a true unit test
    // is harder without more complex testing setups (e.g., capturing stdout).
    // For this tutorial, we'll just ensure it compiles as part of the module.
    #[test]
    fn test_print_value_passed_compiles() {
        print_value_passed(10); // Just checking it can be called
        // No assert needed if we're just checking for compilation within the test context.
    }
}
