// Module: Vector Type
// This module explains vectors (`Vec<T>`) in Rust, which are growable arrays.

pub fn demonstrate_vector_type() {
    println!("\n--- Vector Type ---");

    // Vectors are like arrays but can change in size (dynamic).
    // They are heap-allocated.
    // All elements in a vector must be of the same type.

    // Creating a new empty vector
    let mut vec1: Vec<i8> = Vec::new();
    println!("Initially empty vec1: {:?}", vec1);

    // Pushing elements onto a vector
    vec1.push(7);
    vec1.push(8);
    vec1.push(9);
    println!("vec1 after pushing elements: {:?}", vec1);

    // Creating a vector with initial values using the `vec!` macro
    let vec2: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("vec2 initialized with vec! macro: {:?}", vec2);

    // Accessing elements in a vector
    // Using indexing (can panic if out of bounds)
    let third_element_vec2: i32 = vec2[2]; // Accesses the element 3
    println!("Third element of vec2 (vec2[2]): {}", third_element_vec2);

    // Using the `get()` method (returns an Option<&T>, safer)
    match vec2.get(2) {
        Some(value) => println!("Third element of vec2 (using get): {}", value),
        None => println!("Element at index 2 not found."),
    }

    match vec2.get(10) { // Index 10 is out of bounds
        Some(value) => println!("Element at index 10: {}", value),
        None => println!("Element at index 10 not found (out of bounds). This is expected!"),
    }

    // Modifying elements in a mutable vector
    let mut vec_mut: Vec<String> = Vec::new();
    vec_mut.push(String::from("Hello"));
    vec_mut.push(String::from("World"));
    println!("Original vec_mut: {:?}", vec_mut);

    if let Some(element) = vec_mut.get_mut(0) {
        element.push_str(", Rust!");
    }
    println!("Modified vec_mut: {:?}", vec_mut);

    // Removing elements
    // `pop()` removes the last element and returns it (in an Option)
    if let Some(last_element) = vec_mut.pop() {
        println!("Popped last element from vec_mut: {}", last_element);
    }
    println!("vec_mut after pop: {:?}", vec_mut);

    // Iterating over a vector
    println!("Iterating over vec2 (immutable references):");
    for i in &vec2 {
        println!("Element: {}", i);
    }

    println!("Iterating over vec1 (mutable references and modifying):");
    for val in &mut vec1 {
        *val += 10; // Add 10 to each element
    }
    println!("vec1 after mutable iteration and modification: {:?}", vec1);


    // Length of a vector
    println!("Length of vec1: {}", vec1.len());
    println!("Length of vec2: {}", vec2.len());

    // Vectors can store complex types, like other vectors (for a 2D vector) or structs.
    let vec_of_strings: Vec<String> = vec![
        String::from("Rust"),
        String::from("is"),
        String::from("awesome"),
    ];
    println!("Vector of Strings: {:?}", vec_of_strings);


    // Original prac10.rs code:
    // fn main(){
    //     println!("Vectors in Rust!");
    //     let mut vec1: Vec<i8> = Vec::new(); // Create a new empty vector
    //     let vec2: Vec<i32> = vec![1, 2, 3]; // Create a vector with initial values
    //     vec1.push(7);
    //     vec1.push(8);
    //     vec1.push(9);
    //     println!("{:?}", vec1);
    //     println!("{:?}", vec2);
    //     let mut vrr: Vec<i32> = vec![1, 2, 3, 4, 5];
    //     write_vector(&mut vrr);
    //     println!("{:?}", vrr);
    // }
    // fn write_vector(vec: &mut Vec<i32>){
    //     vec.pop(); // Remove the last element from the vector
    //     vec.push(6);
    //     println!("{:?}", vec);
    // }
    // The refactored module covers these concepts and more.
}
