// Module: References and Dereferencing
// This module explains how to use references to access values and the dereference operator (*).

pub fn demonstrate_references_and_dereferencing() {
    println!("\n--- References and Dereferencing ---");

    let x: u8 = 5;
    let y: &u8 = &x; // y is an immutable reference to x. It holds the memory address of x.

    println!("Value of x: {}", x);
    println!("Value pointed to by y: {}", *y); // Use '*' to dereference y and get the value it points to.
    println!("Value of y itself (memory address of x): {:p}", y);
    println!("Memory address of x: {:p}", &x);
    println!("Memory address of y (where the reference itself is stored): {:p}", &y);

    // Comparing the value and the pointed-to value
    assert_eq!(5, x);
    assert_eq!(5, *y);
    // assert_eq!(x, y); // This would cause a compile error: cannot compare `{integer}` with `&{integer}`

    // Modifying a value through a mutable reference using dereferencing
    let mut z: i32 = 16;
    println!("Initial value of z: {}", z);

    let w: &mut i32 = &mut z; // w is a mutable reference to z.
    *w = *w + 1; // Dereference w to get the value, add 1, then assign back to the dereferenced w.
                 // This modifies the value stored in z.

    // Print the value using *w. This uses the mutable borrow.
    println!("Value pointed to by w after modification: {}", *w);
    // Now that w is not used in a subsequent line, its borrow might be considered ended by NLL,
    // allowing z to be immutably borrowed for printing.
    println!("Value of z after modification (should be same as *w): {}", z);


    // Dereferencing also works with other types, like String
    let s1 = String::from("hello");
    let s2 = &s1; // s2 is an immutable reference to s1

    println!("s1: {}", s1);
    println!("*s2 (dereferenced): {}", *s2); // Dereferencing a &String gives a &str (due to Deref coercion)
                                       // but for basic understanding, it "accesses" the string data.
    println!("Length of s1: {}", s1.len());
    println!("Length using (*s2): {}", (*s2).len()); // Explicit dereference then method call
    println!("Length using s2: {}", s2.len());     // Implicit dereference coercion for method calls

    // Original prac5.rs code:
    // fn main(){
    //     let x:u8 = 5;
    //     let y = &x;
    //     println!("Address = {:p}", &x);
    //     println!("Address = {:p}", y);
    //     let mut z = 16;
    //     let w = &mut z;
    //     *w = *w + 1;
    //     println!("w = {}", w); // Note: This prints the value, not the address, due to Display trait.
    //     println!("z = {}", z);
    // }
    // The refactored code explains these concepts more thoroughly.
}
