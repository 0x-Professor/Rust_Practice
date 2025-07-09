fn main(){
    let x:u8 = 5;
    let y = &x;
    println!("Address = {:p}", &x);
    println!("Address = {:p}", y);
    let mut z = 16;
    let w = &mut z;
    *w = *w + 1;
    println!("w = {}", w);
    println!("z = {}", z);
    
}