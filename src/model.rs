use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Handle {
    pub handle: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub author: String,
    pub timestamp: u128,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEvent {
    pub name: ChatEventType,
    pub identifier: String,
    pub timestamp: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatEventType {
    Joined,
    Left,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Joined {
    pub cid: String,
}
