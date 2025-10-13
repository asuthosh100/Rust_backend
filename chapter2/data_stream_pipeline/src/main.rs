
fn main() {
    let numbers = vec![1,2,3,4,5,6]; 

    //filter() to select values

    //map() to transform them

    //collect() into the new vector

    let proceed : Vec<i32> = numbers
        .into_iter() // turn the vector into an iterator
        .filter(|x| x%2 == 0)
        .map(|x| x*x)
        .collect(); 

    println!("Processed numbers: {:?}", proceed); 

}