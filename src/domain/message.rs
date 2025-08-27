use chrono::{DateTime, Utc};

pub struct Message {
    pub id: u64,
    pub sender_id: u64,
    pub recipient_id: u64,
    pub content: String,
    pub timestamp: DateTime<Utc>
}

impl Message {
    pub fn new(id: u64, sender_id: u64, recipient_id: u64, content: &str) -> Self {
        Self {
            id,
            sender_id,
            recipient_id,
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
}
