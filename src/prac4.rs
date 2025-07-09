fn main(){
    let mut var1:String = String::from("Hello, Blockchain!");
    let w1= &mut var1;
    w1.push_str("Enjoy Learning Rust!");
    println!("W1 is {}", w1);
    let w2= &mut var1;
    w2.push_str("Fixing bugs is fun!");
    println!("W2 is {}", w2);
    //println!("w1 is {} and w2 is {}", w1, w2);
}