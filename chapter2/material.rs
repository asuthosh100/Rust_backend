// OWNERSHIP 
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time. 
    // 3. When the owner goes out of scope, the value will be dropped.


fn main() {
    let s = String::from("hello");
    let t = s; 
   // println!("{}", s); // error: borrow of moved value: `s`
}

// In the above example, when we assign s to t, the ownership is moved not copied. 

// Prevents Use After Free Errors at Compile Time, doublefree errors, dangling pointers. 

//----------------------------------------Borrowing--------------------------------------------------------------------------------
// We can use references to let multiple variables access the same data without taking ownership of it. 
// Rust allows you to borrow data immutably or mutably, but with strict rules. 

// Immutable borrow :

fn print_string(s: &String) {
    println!("{}", s);
}

// Here any number of immutable references are allowed. (SIMILAR TO READ ONLY ACCESS)

// Mutable borrow :

fn print_string(s : &mut String) {
    s.push_str(", world!");
    println!("{}", s);
}
// Here only one mutable reference is allowed at a time. (SIMILAR TO READ AND WRITE ACCESS) 

//Borrowing feature is helpful when it comes to multiple reads and exclusive writes without requiring locks and semaphores.