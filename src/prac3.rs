fn main(){
    let mut var1:String = String::from("Hello");
    modified_string(&mut var1);
    let var2:usize = get_length(&var1);
    println!("length of {} is {} " , var1, var2);

}

fn get_length(a: &String) -> usize{
    let length: usize = a.len();
    return length;
}

fn modified_string(b: &mut String){
    b.push_str(" Blockchain!");
}