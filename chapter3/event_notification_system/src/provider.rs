
#[derive(Debug)]
pub enum NotifyError {
    Timeout,
    Transient(String),
    Permanent(String),
}

pub trait Notifer {
    fn snd(&self, to: &str, message: &str) -> Result <(), NotifyError>; 
}

pub struct WebSocket {

}

impl Notifier for WebSocket {
    fn snd(&self, to: &str, message: &str) -> Result <(), NotifyError> {
        
    }
}