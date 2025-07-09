fn main(){
    println!("Vectors in Rust!");
    let mut vec1: Vec<i8> = Vec::new(); // Create a new empty vector
    let vec2: Vec<i32> = vec![1, 2, 3]; // Create a vector with initial values
    vec1.push(7);
    vec1.push(8);
    vec1.push(9);
    println!("{:?}", vec1);
    println!("{:?}", vec2);
    let mut vrr: Vec<i32> = vec![1, 2, 3, 4, 5];
    write_vector(&mut vrr);
    println!("{:?}", vrr);
}
fn write_vector(vec: &mut Vec<i32>){
    vec.pop(); // Remove the last element from the vector
    vec.push(6);
    println!("{:?}", vec);
    
}