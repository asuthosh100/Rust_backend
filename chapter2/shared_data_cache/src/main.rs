// use RWlock for multiple readers or single writer
// use threads for readers and writers
// use Arc to share safety across threads

// for the data cache we will use a hashmap 

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread; 
use std::time::Duration; 



fn main() {
    let data : HashMap<String, String> = HashMap::new(); 
    let shared_data = Arc::new(RwLock::new(data)); 

    let writer_data = Arc::clone(&shared_data); 

    let _writer = thread::spawn(move || {

        loop {
            
            {

            
            let mut map = writer_data.write().unwrap(); // .write().unwrap() looks for exclusive write access
            map.insert("counter".to_string(), "updated".to_string()); 
            println!("[Writer] updated the key 'Counter'");

            }

            thread::sleep(Duration::from_secs(1)); 


        }
            
        });

         let mut handles = vec![]; // 

        for i in 0..5 {
            let reader_data= Arc::clone(&shared_data); // clone the pointer
            let handle = thread::spawn(move || {

                loop {
                        {

                        let map = reader_data.read().unwrap(); // .write().unwrap() looks for exclusive write access
                        let exists = map.contains_key("counter"); 
                        println!("[Reader{i}] Key 'counter' exists? {exists}"); 

                        }

                        thread::sleep(Duration::from_millis(50)); 
                    }
                        
            });
                
        

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

       
}
