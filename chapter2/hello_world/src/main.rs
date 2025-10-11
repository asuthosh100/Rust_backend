// --------------------------------STEP 1 : Project Setup--------------------------------

use std::sync::mpsc; // multiple producer, single consumer
use std::sync::Arc; // atomic reference counting
use std::thread;
use std::time::Duration; // for simulating work with sleep

//--------------------------------STEP 2 : Define the logger thread-------------------------------

// parameter is a receiver that gets messages from other threads
fn start_logger(rx: mpsc::Receiver<String>) { // example of how such a reciver is created => let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || { // thread::spawn creates a new thread and move keyword transfers the ownership of rx to the new thread
        for msg in rx {
            println!("[LOGGER] {}", msg);
        }
    });
}


// --------------------------------STEP 3 : Create the channel and worker threads-------------------------------
fn main() {
    // Create a channel for communication between threads
    let (tx, rx) = mpsc::channel(); // here tx is the transmitter and rx is the receiver

    // start the logger thread
    start_logger(rx); // only the logger thread owns rx, hence single consumer. 
    // logger thread loops over the incoming messages and prints them to the console.

    let tx = Arc::new(tx);

    let mut handles = vec![];

    for i in 0..5{
        let tx = Arc::clone(&tx); //
        let handle =  thread::spawn(move || { // thread::spawn creates a new thread and move keyword transfers the ownership of rx to the new thread
        for j in 0..10 {
            let msg = format!("Message {} from thread {}", j, i);
            tx.send(msg).unwrap(); // send method sends the message to the receiver.
            thread::sleep(Duration::from_millis(50)); // simulate some work
        }
    });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }   

    thread::sleep(Duration::from_secs(1)); // give some time for the logger to finish processing messages before main thread exits

}
