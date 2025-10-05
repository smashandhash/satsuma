use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub id: u64,
    pub sender_public_key: String,
    pub recipient_public_key: String,
    pub content: String,
    pub timestamp: DateTime<Utc>
}

impl Message {
    pub fn new(id: u64, sender_public_key: &str, recipient_public_key: &str, content: &str) -> Self {
        Self {
            id,
            sender_public_key: sender_public_key.to_string(),
            recipient_public_key: recipient_public_key.to_string(),
            content: content.to_string(),
            timestamp: Utc::now(),
        }
    }

    pub fn edit_content(&mut self, new_content: &str) {
        self.content = new_content.to_string();
    }

    pub fn is_empty(&self) -> bool {
        self.content.trim().is_empty()
    }

    pub fn is_to_self(&self) -> bool {
        self.sender_public_key == self.recipient_public_key
    }
}
