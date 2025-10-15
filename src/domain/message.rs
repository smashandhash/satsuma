use crate::domain::event_kind::EventKind;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use hex;

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub id: String,
    pub sender_public_key: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub kind: EventKind
}

impl Message {
    pub fn new(sender_public_key: &str, content: &str, kind: EventKind) -> Self {
        let timestamp = Utc::now();
        let formatted_id = format!("{}{}{}{}", sender_public_key.to_string(), timestamp.clone(), kind.clone() as u32, content.to_string());
        Self {
            id: hex::encode(Sha256::digest(formatted_id.as_bytes())),
            sender_public_key: sender_public_key.to_string(),
            content: content.to_string(),
            timestamp: timestamp.clone(),
            kind
        }
    }

    pub fn edit_content(&mut self, new_content: &str) {
        self.content = new_content.to_string();
    }

    pub fn is_empty(&self) -> bool {
        self.content.trim().is_empty()
    }
}
