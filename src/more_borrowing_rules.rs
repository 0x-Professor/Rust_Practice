// Module: More Borrowing Rules
// This module further explores Rust's borrowing rules, especially concerning mutable references.

pub fn demonstrate_more_borrowing_rules() {
    println!("\n--- More Borrowing Rules (Mutable Borrows) ---");

    let mut s = String::from("Hello");

    // Scope of a mutable borrow
    // You can have multiple mutable references, but not simultaneously.
    // Their scopes must not overlap.

    {
        let r1 = &mut s;
        r1.push_str(", world");
        println!("Inside inner scope, r1 (s): \"{}\"", r1);
    } // r1 goes out of scope here, so the mutable borrow ends.

    // Now that the first mutable borrow (r1) is out of scope, we can create another one.
    {
        let r2 = &mut s;
        r2.push_str("!");
        println!("Inside another inner scope, r2 (s): \"{}\"", r2);
    } // r2 goes out of scope here.

    println!("After all modifications, s: \"{}\"", s);

    // The original prac4.rs had an example that would not compile if the println! was uncommented:
    // fn main(){
    //     let mut var1:String = String::from("Hello, Blockchain!");
    //     let w1= &mut var1;
    //     w1.push_str("Enjoy Learning Rust!");
    //     println!("W1 is {}", w1); // w1 is used here
    //     let w2= &mut var1; // This would be an error if w1 was still "live"
    //     w2.push_str("Fixing bugs is fun!");
    //     println!("W2 is {}", w2);
    //     //println!("w1 is {} and w2 is {}", w1, w2); // This would DEFINITELY be an error
    // }
    // The compiler is smart enough to see that if `w1` is not used after `w2` is created,
    // it can sometimes allow it (Non-Lexical Lifetimes - NLL).
    // However, using both `w1` and `w2` after `w2`'s creation (like in the commented println)
    // would be a clear violation of having two simultaneous mutable borrows.

    let mut var1 = String::from("Initial String");
    println!("Original var1: \"{}\"", var1);

    let w1 = &mut var1; // First mutable borrow
    w1.push_str(" - Modified by w1");
    // If we print w1 here, its borrow is still active.
    // println!("w1: {}", w1); // Uncommenting this makes the next line an error with some Rust versions or specific NLL interpretations.

    // The key is that `w1`'s borrow must END before `w2`'s borrow begins,
    // OR `w1` must not be used after `w2` is created if their scopes overlap.
    // Rust's Non-Lexical Lifetimes (NLL) allow `w1`'s borrow to be considered "ended"
    // if it's not used again, even if it's technically still in scope.

    // To be safe and clear for a tutorial, it's best to show distinct scopes or sequential usage.
    // The example with `s` and inner scopes above is clearer.

    // Let's re-illustrate the problematic part from prac4 more directly:
    let mut conflict_string = String::from("Try to conflict");
    let c1 = &mut conflict_string;
    // let c2 = &mut conflict_string; // ERROR: cannot borrow `conflict_string` as mutable more than once at a time
    // c1.push_str(" via c1");
    // c2.push_str(" via c2");
    // println!("c1: {}, c2: {}", c1, c2);
    c1.push_str(" - this is fine as c2 is commented out");
    println!("c1 (conflict_string): \"{}\"", c1);
    println!("(Example of conflicting mutable borrows is commented out to allow compilation)");
}
