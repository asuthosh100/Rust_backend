use std::net::SocketAddr;

#[derive(Debug, Clone)]

pub struct ChatMessage {
    pub sender: SocketAddr, 
    pub content: String,
}