// u

// fn read_lines(filename: &str)-> Result<Vec<String>, Error>{

// }

use std::fs::File; // for file handling (Open, create, close)
// use std::io::Write;  
// use std::io::Read; 
use std::io::Error; 
use std::io::{self, BufRead, BufReader};

fn read_lines(filename: &str) -> Result<Vec<String>, Error> {

    let file = File::open(filename)?; 
    let reader = BufReader::new(file); 

    let mut lines = Vec::new(); 
    for line in reader.lines() {
        //println!("Line: {}", line?);
        lines.push(line?); 
    }

    Ok(lines)
}

// fn main() -> io::Result<()> {
//     let file = File::open("example.txt")?;
//     let reader = BufReader::new(file);

//     for line in reader.lines() {
//         println!("Line: {}", line?);
//     }

//     Ok(())
// }


fn main() -> io::Result<()> {
    let lines = read_lines("example.txt")?;

    for (i, line) in lines.iter().enumerate() { // loop over items without taking ownership
        println!("Line {}: {}", i + 1, line);
    }

    Ok(())
}
