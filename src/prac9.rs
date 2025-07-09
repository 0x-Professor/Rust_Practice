fn main(){
    println!("Array's Practice!");
    let arr1: [u8; 5]; // Declare an array of 5 elements of type u8
    arr1 = [1,2,3,4,5];
    println!("{:#?}", arr1);
    let arr2: [u8; 5] = [6,7,8,9,10]; //initialization of an array with values
    let sum: u8 = arr1[0] + arr2[1];
    println!("Sum of the elements of the array is {}", sum);
    let _arr3: [u8; 5];
}