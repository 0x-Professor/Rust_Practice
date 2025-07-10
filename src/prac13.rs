fn main(){
    let mut counter:u8 = 0;
    loop{
        counter+=1;
        if counter == 5 {
            println!("Counter reached 5, exiting loop.");
            break;
        }
    }
    func_while();
    func_for();
}
fn func_while(){
    let mut counter:u8 =0;
    while counter !=5{
        counter +=1;
        println!("Counter in while loop: {}", counter);
    }
}

fn func_for(){
    for i in 1..5{
        println!("Counter in for loop: {}", i);
    }
    let arr:[&str;5] = ["Hello", "Blockchain", "Rust", "Programming", "Language"];
    for element in arr.iter(){
        println!("Array element: {}", element);
    }
}