// Declare the modules
mod basic_syntax;
mod string_ownership;
mod references_borrowing;
mod more_borrowing_rules;
mod references_and_dereferencing;
mod floating_point_types;
mod boolean_type;
mod char_type;
mod array_type;
mod vector_type;
mod control_flow;
mod loops;
mod user_input; // Added this module
// prac11.rs has been merged into basic_syntax.rs

fn main() {
    println!("Welcome to the Rust Tutorial!");

    // Call a function from the basic_syntax module
    basic_syntax::demonstrate_basics();

    // Call a function from the string_ownership module
    string_ownership::demonstrate_string_ownership();

    // Call a function from the references_borrowing module
    references_borrowing::demonstrate_references_and_borrowing();

    // Call a function from the more_borrowing_rules module
    more_borrowing_rules::demonstrate_more_borrowing_rules();

    // Call a function from the references_and_dereferencing module
    references_and_dereferencing::demonstrate_references_and_dereferencing();

    // Call a function from the floating_point_types module
    floating_point_types::demonstrate_floating_point_types();

    // Call a function from the boolean_type module
    boolean_type::demonstrate_boolean_type();

    // Call a function from the char_type module
    char_type::demonstrate_char_type();

    // Call a function from the array_type module
    array_type::demonstrate_array_type();

    // Call a function from the vector_type module
    vector_type::demonstrate_vector_type();

    // Call a function from the control_flow module
    control_flow::demonstrate_control_flow();

    // Call a function from the loops module
    loops::demonstrate_loops();

    // Call a function from the user_input module
    // NOTE: This module will pause execution and wait for user input.
    // To run through other modules quickly without typing, you might want to comment this line out temporarily.
    user_input::demonstrate_user_input();


    // The old code from main.rs, which can also be a module or integrated elsewhere.
    // For now, I'll keep it here and we can decide where it fits best.
    println!("\n--- Original Main.rs Demonstrations ---");
    let string_literal:&str = "Hello, Blockchain!";
    println!("{}", string_literal);
    let mut string_var:String = String::from("Happy Coding!");
    println!("{}", string_var);
    string_var.push_str(" Enjoy Rust!");
    println!("{}", string_var);
    let employee_info:(&str, u8) = ("Mazhar", 20);
    let (name, age) = employee_info;
    println!("{} is {} years old.", name, age);
    print_value_original_main(7);
    let sum:u8 = add_original_main(12, 18);
    println!("Sum: {}", sum);
}

// These functions were originally in main.rs.
// They are kept here for now. We might move them to a more appropriate module later.
fn print_value_original_main(value:u8){
    println!("Value from original main: {}", value);
}

fn add_original_main(x:u8, y:u8) -> u8 {
    x + y // Implicit return
}
