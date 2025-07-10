fn main(){
    let number:u8 = 10;
    if number <10{
        println!("number is less than 10");
    }
    else if number >10{
        println!("number is greater than 10");
    }
    else{
        println!("number is equal to 10");
    }
    func_match();
}

fn func_match(){
    let number:u8 = 3;
    match number{
        0 => println!("number is 0"),
        1 => println!("number is 1"),
        2 => println!("number is 2"),
        3 => println!("number is 3"),
        4 => println!("number is 4"),
        5 => println!("number is 5"),
        _ => println!("number is not 0-5")
    }
}