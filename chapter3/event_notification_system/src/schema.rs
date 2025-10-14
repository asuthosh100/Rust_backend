use serde::{Serialize, Deserialize}; 


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChannelType{
    SMS, 
    Email, 
    WebSocket, 
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotificationEvent {
    pub id: String, 
    pub user_id: String, 
    pub channel: ChannelType, 
    pub payload: String, 
    pub timestamp: u64,
}