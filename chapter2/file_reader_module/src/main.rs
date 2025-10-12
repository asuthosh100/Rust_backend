// use std::io::Error; 

// fn read_lines(filename: &str)-> Result<Vec<String>, Error>{

// }

use std::fs::File; // for file handling (Open, create, close)
use std::io::Write;  

fn main() {
    let mut file = File::create("example.txt").expect("Could not create file"); // create a new price

    file.write_all(b"Hello, world!").expect("Could not write to file"); // write some data to the file

    println!("File created and data written successfully.");
}
