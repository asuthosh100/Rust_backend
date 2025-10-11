//---------------------------------------------------------------OWNERSHIP-------------------------------------------------------- 
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

//--------------------------------------------------------------Borrowing------------------------------------------------------------
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


//------------------------------------------------------------Lifetimes--------------------------------------------------------------

//Ownership and borrowing cover the who and how of data usage, but not the when.
//Lifetimes are a way to ensure that references are valid as long as they are used 

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
} 

// Here 'a is a lifetime parameter that tells the compiler that the returned reference will live as long as both input references.

//Lifetimes become especially important in structs with references , trait implementations , and function return values  that involve borrowed data.


//------------------------------------------------------Traits----------------------------------------------------------------------

// Key feature of Rust's type system that enables polymorphism and code reuse.

// Basically a trait for a type. For example, we can define a trait called Summary that requires a summarize method.
// and then implement that trait for different types like NewsArticle and Tweet.

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    fn summarize(&self) -> String;
}

// Use cases of traits include defining shared behavior across different types, enabling polymorphism, and creating abstractions that can work with any type that implements a specific trait.


//------------------------------------------------------ENUM and Pattern Matching------------------------------------------------------------


// Enum lets you define a type by enumerating its possible variants.

// Rust enums can store data with each variant — something impossible in plain C.


enum Message {
    Quit,                       // no data
    Move { x: i32, y: i32 },    // struct-like
    Write(String),              // tuple-like
    ChangeColor(u8, u8, u8),    // multiple values
}

// Pattern matching is extensively used with enums to execute different code based on the variant.