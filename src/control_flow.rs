// Module: Control Flow
// This module explains `if/else if/else` expressions and `match` statements in Rust.

pub fn demonstrate_control_flow() {
    println!("\n--- Control Flow: if/else and match ---");

    // --- if/else if/else expressions ---
    // `if` expressions allow you to branch your code depending on conditions.
    // Conditions must be of type `bool`.

    let number: u8 = 10;
    println!("Demonstrating if/else with number = {}", number);

    if number < 5 {
        println!("Condition: number < 5 is true. Number is small.");
    } else if number > 20 {
        println!("Condition: number > 20 is true. Number is large.");
    } else if number == 10 {
        println!("Condition: number == 10 is true. Number is exactly 10!");
    } else {
        // This `else` block would catch numbers >= 5, <= 20, and not equal to 10.
        println!("Condition: number is between 5 and 20 (inclusive), but not 10.");
    }

    // `if` is an expression, meaning it can return a value.
    // All blocks in an `if/else` expression must return the same type.
    let condition = true;
    let result_from_if = if condition {
        "Condition was true" // This is a &str
    } else {
        "Condition was false" // This is also a &str
        // 0 // This would cause a compile error: `if` and `else` have incompatible types
    };
    println!("Result from if expression: {}", result_from_if);

    // --- match statements ---
    // `match` allows you to compare a value against a series of patterns and execute code
    // based on which pattern matches. Patterns can be literal values, variable names, wildcards, etc.
    // `match` is exhaustive: you must cover every possible case for the value being matched.

    let value_to_match: u8 = 3;
    println!("\nDemonstrating match with value_to_match = {}", value_to_match);

    match value_to_match {
        0 => println!("Match: value is Zero"),
        1 => println!("Match: value is One"),
        2 | 3 => { // Multiple patterns can be combined with `|`
            println!("Match: value is Two or Three");
            // Match arms can be blocks of code
        }
        4..=6 => { // `a..=b` is an inclusive range pattern (matches 4, 5, or 6)
            println!("Match: value is between Four and Six (inclusive)");
        }
        _ => println!("Match: value is something else (default case using wildcard `_`)"),
    } // The `_` (underscore) is a wildcard pattern that matches any value and is often used for default cases.

    // `match` is also an expression and can return a value.
    let food = "apple";
    let fruit_type = match food {
        "apple" => "It's a crunchy fruit!",
        "banana" => "It's a soft fruit!",
        "orange" | "lemon" => "It's a citrus fruit!",
        _ => "It's some other kind of food.",
    };
    println!("Matching food '{}': {}", food, fruit_type);

    // Matching Option<T>
    let some_number: Option<i32> = Some(5);
    let no_number: Option<i32> = None;

    println!("Matching an Option<i32> (Some(5)):");
    match some_number {
        Some(n) => println!("Got a number: {}", n),
        None => println!("Got nothing!"),
    }

    println!("Matching an Option<i32> (None):");
    match no_number {
        Some(n) => println!("Got a number: {}", n), // This arm won't be hit
        None => println!("Got nothing! (This is expected for no_number)"),
    }

    // Original prac12.rs:
    // fn main(){
    //     let number:u8 = 10;
    //     if number <10{
    //         println!("number is less than 10");
    //     }
    //     else if number >10{
    //         println!("number is greater than 10");
    //     }
    //     else{
    //         println!("number is equal to 10");
    //     }
    //     func_match();
    // }
    // fn func_match(){
    //     let number:u8 = 3;
    //     match number{
    //         0 => println!("number is 0"),
    //         1 => println!("number is 1"),
    //         2 => println!("number is 2"),
    //         3 => println!("number is 3"),
    //         4 => println!("number is 4"),
    //         5 => println!("number is 5"),
    //         _ => println!("number is not 0-5")
    //     }
    // }
    // The refactored module expands on these concepts.
}

#[cfg(test)]
mod tests {
    use super::*; // Import items from the outer module

    #[test]
    fn test_if_expression_true_case() {
        // Recreate the logic for the if expression that returns a value
        let condition = true;
        let result = if condition {
            "Condition was true"
        } else {
            "Condition was false"
        };
        assert_eq!(result, "Condition was true");
    }

    #[test]
    fn test_if_expression_false_case() {
        let condition = false;
        let result = if condition {
            "Condition was true"
        } else {
            "Condition was false"
        };
        assert_eq!(result, "Condition was false");
    }

    #[test]
    fn test_match_food_apple() {
        // Recreate or call a helper for the match logic if it were more complex.
        // For this direct example:
        let food = "apple";
        let fruit_type = match food {
            "apple" => "It's a crunchy fruit!",
            "banana" => "It's a soft fruit!",
            "orange" | "lemon" => "It's a citrus fruit!",
            _ => "It's some other kind of food.",
        };
        assert_eq!(fruit_type, "It's a crunchy fruit!");
    }

    #[test]
    fn test_match_food_banana() {
        let food = "banana";
        let fruit_type = match food {
            "apple" => "It's a crunchy fruit!",
            "banana" => "It's a soft fruit!",
            "orange" | "lemon" => "It's a citrus fruit!",
            _ => "It's some other kind of food.",
        };
        assert_eq!(fruit_type, "It's a soft fruit!");
    }

    #[test]
    fn test_match_food_other() {
        let food = "carrot";
        let fruit_type = match food {
            "apple" => "It's a crunchy fruit!",
            "banana" => "It's a soft fruit!",
            "orange" | "lemon" => "It's a citrus fruit!",
            _ => "It's some other kind of food.",
        };
        assert_eq!(fruit_type, "It's some other kind of food.");
    }

    #[test]
    fn test_match_option_some() {
        let some_number: Option<i32> = Some(5);
        // Assign the result of the match expression directly to description
        let description = match some_number {
            Some(n) => format!("Got a number: {}", n),
            None => String::from("Got nothing!"),
        };
        assert_eq!(description, "Got a number: 5");
    }

    #[test]
    fn test_match_option_none() {
        let no_number: Option<i32> = None;
        // Assign the result of the match expression directly to description
        let description = match no_number {
            Some(n) => format!("Got a number: {}", n),
            None => String::from("Got nothing! (This is expected for no_number)"),
        };
        assert_eq!(description, "Got nothing! (This is expected for no_number)");
    }
}
