fn main(){
    println!("Array's Practice!");
    let arr1: [u8; 5]; // Declare an array of 5 elements of type u8
    arr1 = [1,2,3,4,5];
    println!("{:#?}", arr1);
    let arr2: [u8; 5] = [6,7,8,9,10]; //initialization of an array with values
    let sum: u8 = arr1[0] + arr2[1];
    println!("Sum of the elements of the array is {}", sum);
    let _arr3: [u8; 5];
    let mut arr: [&str; 2] = ["Hello", "Blockchain!"]; // Declare an array of 2 elements of type &str
    write_array(arr);
    write_array2(&mut arr);
    println!("original Array {:#?}", arr);
    
}
fn write_array(mut arr1: [&str; 2]){
    arr1[0] = "Hello, Rust!";
    arr1[1] = "Welcome to Blockchain!";
    println!("Modified array: {:#?}", arr1);
}
// Another way to write the array without creating their clone

fn write_array2(arr2:&mut [&str; 2] ){
    arr2[0] = "Hello, Buddy!";
    arr2[1] = "Enjoy Blockchain!";
    println!("Modified array2: {:#?}", arr2);
}