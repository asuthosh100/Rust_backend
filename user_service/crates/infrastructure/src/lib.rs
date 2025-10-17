use domain::{User, DomainError}; 
use std::collections::HashMap; 
use std::sync::RwLock; 

pub struct InMemoryUserRepo {
    store: RwLock<HashMap<String, User>> , 
}

impl InMemoryUserRepo {
    pub fn new() -> Self {
        Self {
            store: RwLock::new(HashMap::new()), 
        }
    }

    pub fn save(&self, user: User)-> Result<(), DomainError> {
        let mut map = self.store.write().unwrap(); 

        if map.contains_key(&user.username) {
            return Err(DomainError::DuplicateUser); 
        }

        map.insert(user.username.clone(), user); 
        Ok(())
    }

    pub fn find(&self, username: &str)-> Option<User> {
        let map = self.store.read().unwrap(); 
        map.get(username).cloned()
    }


}