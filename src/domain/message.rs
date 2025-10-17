use super::{
    services::generate_event_id::generate_event_id,
    event_kind::EventKind
};
use chrono::Utc;

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub id: String,
    pub sender_public_key: String,
    pub content: String,
    pub created_at: u64,
    pub kind: EventKind,
    pub tags: Vec<Vec<String>>
}

impl Message {
    pub fn new(sender_public_key: &str, content: &str, kind: EventKind, tags: Vec<Vec<String>>) -> Self {
        let created_at = Utc::now().timestamp() as u64;
        let id = generate_event_id(sender_public_key, created_at.clone(), kind.clone() as u32, &tags, content);
        Self {
            id,
            sender_public_key: sender_public_key.to_string(),
            content: content.to_string(),
            created_at: created_at.clone(),
            kind,
            tags
        }
    }

    pub fn edit_content(&mut self, new_content: &str) {
        self.content = new_content.to_string();
    }

    pub fn is_empty(&self) -> bool {
        self.content.trim().is_empty()
    }
}
