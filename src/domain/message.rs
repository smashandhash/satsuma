use super::event_kind::EventKind;

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
    pub fn new(id: String, sender_public_key: String, content: String, created_at: u64, kind: EventKind, tags: Vec<Vec<String>>) -> Self {
        Self {
            id,
            sender_public_key,
            content,
        created_at,
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
