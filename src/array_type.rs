// Module: Array Type
// This module explains arrays in Rust.

pub fn demonstrate_array_type() {
    println!("\n--- Array Type ---");

    // Arrays in Rust have a fixed size, known at compile time.
    // All elements in an array must be of the same type.
    // Syntax: [type; size]

    // Declare an array of 5 u8 integers.
    let arr1: [u8; 5];
    // Initialize the array.
    arr1 = [1, 2, 3, 4, 5];
    println!("arr1: {:?}", arr1); // Using {:?} (debug print) for arrays.

    // Declare and initialize an array simultaneously.
    let arr2: [u8; 5] = [6, 7, 8, 9, 10];
    println!("arr2: {:?}", arr2);

    // Accessing array elements using indexing (0-based).
    let sum_elements: u8 = arr1[0] + arr2[1]; // 1 + 7 = 8
    println!("Sum of arr1[0] + arr2[1]: {}", sum_elements);

    // Initialize an array with all elements set to the same value.
    let arr3: [i32; 10] = [0; 10]; // Creates an array of ten 0s.
    println!("arr3 (initialized to all zeros): {:?}", arr3);

    // Arrays are stack-allocated by default if their size is known at compile time.
    // Trying to access an out-of-bounds index will cause a panic at runtime.
    // e.g., `arr1[10]` would panic.

    // Modifying array elements (if the array is mutable).
    let mut arr_mut: [i32; 3] = [10, 20, 30];
    println!("Original arr_mut: {:?}", arr_mut);
    arr_mut[1] = 25;
    println!("Modified arr_mut: {:?}", arr_mut);

    // Iterating over an array
    println!("Iterating over arr_mut:");
    for element in arr_mut.iter() {
        println!("Element: {}", element);
    }

    // Getting the length of an array
    println!("Length of arr1: {}", arr1.len());
    println!("Length of arr_mut: {}", arr_mut.len());

    // Passing arrays to functions
    // Arrays are passed by value (copied) if they implement the Copy trait.
    // For primitive types, this is often the case.
    // For larger arrays, or arrays of non-Copy types, it's common to pass slices (&[T]).
    let str_arr: [&str; 2] = ["Hello", "Rust"];
    print_str_array_by_value(str_arr); // str_arr is copied
    println!("Original str_arr after by-value call: {:?}", str_arr); // Unchanged

    let mut modifiable_str_arr: [&str; 2] = ["Initial", "Value"];
    print_str_array_by_slice(&modifiable_str_arr); // Passed as an immutable slice
    modify_str_array_by_mutable_slice(&mut modifiable_str_arr); // Passed as a mutable slice
    println!("Modified modifiable_str_arr: {:?}", modifiable_str_arr);


    // Slices: A slice is a reference to a contiguous sequence of elements in a collection,
    // rather than the whole collection. Slices don't have ownership.
    let int_array: [i32; 5] = [1, 2, 3, 4, 5];
    let slice_of_int_array: &[i32] = &int_array[1..3]; // Slice from index 1 up to (but not including) 3 -> [2, 3]
    println!("Slice of int_array (elements at index 1 and 2): {:?}", slice_of_int_array);


    // Original prac9.rs functions (write_array, write_array2) are covered by the examples above.
    // fn write_array(mut arr1: [&str; 2]){ // This took by value (copied)
    //     arr1[0] = "Hello, Rust!";
    //     arr1[1] = "Welcome to Blockchain!";
    //     println!("Modified array: {:#?}", arr1);
    // }
    // fn write_array2(arr2:&mut [&str; 2] ){ // This took by mutable reference
    //     arr2[0] = "Hello, Buddy!";
    //     arr2[1] = "Enjoy Blockchain!";
    //     println!("Modified array2: {:#?}", arr2);
    // }
}

fn print_str_array_by_value(mut arr: [&str; 2]) {
    // Modifying `arr` here only affects the local copy.
    arr[0] = "Changed";
    println!("Inside print_str_array_by_value (local copy modified): {:?}", arr);
}

fn print_str_array_by_slice(slice: &[&str]) {
    println!("Inside print_str_array_by_slice (immutable slice): {:?}", slice);
    // slice[0] = "Cannot change"; // This would be an error as slices are immutable by default
}

fn modify_str_array_by_mutable_slice(slice: &mut [&str]) {
    // This function takes a mutable slice, so it can modify the original array's elements.
    if !slice.is_empty() {
        slice[0] = "Modified";
    }
    if slice.len() > 1 {
        slice[1] = "Slice";
    }
    println!("Inside modify_str_array_by_mutable_slice: {:?}", slice);
}
