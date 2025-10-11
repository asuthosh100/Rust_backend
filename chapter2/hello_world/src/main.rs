// --------------------------------STEP 1 : Project Setup--------------------------------

use std::sync::mpsc; // multiple producer, single consumer
use std::sync:Arc; // atomic reference counting
use std::thread;
use std::time::Duration;

//--------------------------------STEP 2 : Define the logger thread-------------------------------

// parameter is a receiver that gets messages from other threads
fn start_logger(rx: mpsc::Receiver<String>) { // example of how such a reciver is created => let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || { // thread::spawn creates a new thread and move keyword transfers the ownership of rx to the new thread
        for msg in rx {
            println!("Log: {}", msg);
        }
    });
}



fn main() {
    println!("Hello, world!");
}
