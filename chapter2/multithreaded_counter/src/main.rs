


// Multithreaded Counter with Mutex 

// Shared counter incremented across multiple threads

// Use Arc<Mutex<i32>> to wrap the counter
// spwan 10 threads; each thread should increment the counter 100 times
// wait for all threads to finish using join()
// print the final counter value


use std::sync::{Arc, Mutex}; 
use std::thread; 

fn main() {
    //Create a shared counter using Arc<Mutex<i32>> 
    // Think of Arc<Mutex<T>> like a thread-safe smart pointer to a locked box

    let counter = Arc::new(Mutex::new(0)); 

    // 0 → the initial counter value
    // Mutex::new(0) → wraps it in a mutex for safe mutation (interior mutability)
    // Arc::new(...) → wraps that in an atomic reference counter so it can be shared between threads

    let mut handles = vec![]; // 

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // clone the pointer
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 100;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", *counter.lock().unwrap());

}

