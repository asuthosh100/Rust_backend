// Main focus : Lifetime annotations


fn first_word<'a>(s: &'a str) -> &'a str {

    match s.find(' ') {
        Some(index) => &s[..index],
        None => s, 
    }
}

fn main() {
    let msg = "hello world"; 
    let word = first_word(msg); 
    println!("First word: {}", word); 
}