// use RWlock for multiple readers or single writer
// use threads for readers and writers
// use Arc to share safety across threads

// for the data cache we will use a hashmap 

use std::collections::Hashmap;
use std::sync::{Arc, RwLock};

fn main() {
    let data : Hashmap<String, String> = Hashmap::new(); 
    let shared_data = Arc::new(RwLock::new(data)); 
}
