fn main(){
    let x:u8 = 5;
    println!("value of x in main funciton is {}", x);
    print_function(x);
}

fn print_function(value:u8){
    println!("value of x in print_function is {}", value);
}