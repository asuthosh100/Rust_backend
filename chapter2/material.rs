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