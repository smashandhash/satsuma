use super::event::Event;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Message {
    pub id: String,
    pub public_key: String,
    pub content: String,
    pub created_at: u64,
    pub kind: u32,
    pub tags: Vec<Vec<String>>,
    pub signature: String
}

impl Message {
    pub fn new(id: String, public_key: String, content: String, created_at: u64, kind: u32, tags: Vec<Vec<String>>, signature: String) -> Self {
        Self {
            id,
            public_key,
            content,
            created_at,
            kind,
            tags,
            signature
        }
    }

    pub fn edit_content(&mut self, new_content: &str) {
        self.content = new_content.to_string();
    }

    pub fn is_empty(&self) -> bool {
        self.content.trim().is_empty()
    }
}

impl From<Event> for Message {
    fn from(event: Event) -> Self {
        Self {
            id: event.id,
            public_key: event.public_key,
            content: event.content,
            created_at: event.created_at,
            kind: event.kind,
            tags: event.tags,
            signature: event.signature
        }
    }
}
