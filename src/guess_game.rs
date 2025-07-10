use rand::thread_rng;
use rand::seq::SliceRandom;
use std::io;
fn main(){
    let mut rng = thread_rng();
    let mut input = String::new();
    println!("Welcome to the Guessing Game!");
    let choices = ["rock", "paper", "scissors"];
    let random_choice = match choices.choose(&mut rng) {
        Some(choice) => {
            println!("Computer has chose now it's your turn!");
            choice
        },
        None => {
            println!("Failed to choose a valid option.");
            return;
        }
    };
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let user_choice = input.trim().to_lowercase();
    if choices.contains(&user_choice.as_str()) {
        println!("You chose: {}", user_choice);
        if user_choice == "rock" && *random_choice == "scissors" ||
           user_choice == "paper" && *random_choice == "rock" ||
           user_choice == "scissors" && *random_choice == "paper" {
            println!("You win!");
        } else if user_choice.as_str() == *random_choice {
            println!("It's a tie!");
        } else {
            println!("You lose!");
        }
    } else {
        println!("Invalid choice. Please choose rock, paper, or scissors.");
    }
}
