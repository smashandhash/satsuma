#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub id: String,
    pub session_id: String, // DO NOT INCLUDE THIS ON THE NOSTR PROTOCOL
    pub public_key: String,
    pub content: String,
    pub created_at: u64,
    pub kind: MessageKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageKind {
    Direct,
    Channel,
    Group,
    Thread(String)
}

impl Message {
    pub fn new(id: String, session_id: String, public_key: String, content: String, created_at: u64, kind: MessageKind) -> Self {
        Self {
            id,
            session_id,
            public_key,
            content,
            created_at,
            kind,
        }
    }

    pub fn edit_content(&mut self, new_content: &str) {
        self.content = new_content.to_string();
    }

    pub fn is_empty(&self) -> bool {
        self.content.trim().is_empty()
    }
}
