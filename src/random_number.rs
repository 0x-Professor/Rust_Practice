use rand::Rng;
fn main(){
    let mut rng = rand::thread_rng();
    let random_number:u32 = rng.gen_range(1..100);
    println!("{}",random_number);
    let random_int:i32 = rng.r#gen();
    println!("{}",random_int);
    let random_float:f64 = rng.r#gen();
    println!("{}",random_float);
    let random_bool:bool = rng.r#gen();
    println!("{}",random_bool);
}