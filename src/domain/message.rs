#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub id: String,
    pub public_key: String,
    pub content: String,
    pub created_at: u64,
    pub kind: MessageKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageKind {
    Direct(String),
    Channel(String),
    Group(String),
    Thread(String)
}

impl Message {
    pub fn new(id: String, public_key: String, content: String, created_at: u64, kind: MessageKind) -> Self {
        Self {
            id,
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
