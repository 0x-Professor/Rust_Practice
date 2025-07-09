fn main(){
    let is_sunny:bool = true;
    let is_raining = false; // by default, it is bool
    let need_umbrella = is_sunny && is_raining;
    let need_sunglasses = is_sunny || is_raining;
    println!("need_umbrella: {}", need_umbrella);
    println!("need_sunglasses: {}", need_sunglasses);
}