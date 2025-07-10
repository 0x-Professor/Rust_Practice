mod prac1;
mod prac3;
mod prac4;
mod prac5;
mod prac6;
mod prac7;
mod prac8;
mod prac9;
mod prac10;
mod prac11;
mod prac12;
mod prac13;
mod prac14;

fn main(){
    let string_literal:&str = "Hello, Blockchain!";
    println!("{}", string_literal);
    let mut string_var:String = String::from("Happy Coding!");
    println!("{}", string_var);
    string_var.push_str(" Enjoy Rust!");
    println!("{}", string_var);
    let employee_info:(&str, u8) = ("Mazhar", 20);
    let (name, age) = employee_info;
    println!("{} is {} years old.", name, age);
    print_value(7);
    let sum:u8 = add(12, 18);
    println!("Sum: {}", sum);
}
fn print_value(value:u8){
    println!("Value: {}", value);
    
}
fn add(x:u8, y:u8) -> u8 {
    return x + y
    
}