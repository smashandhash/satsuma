use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Message {
    pub id: String,
    pub public_key: String,
    pub content: String,
    pub created_at: u64,
    pub kind: MessageKind,
    pub tags: Vec<Vec<String>>,
    pub signature: String,
    pub thread_id: Option<String>
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum MessageKind {
    Direct(String),
    Channel(String),
    Group(String)
}

impl Message {
    pub fn new(id: String, public_key: String, content: String, created_at: u64, kind: MessageKind, tags: Vec<Vec<String>>, signature: String) -> Self {
        Self {
            id,
            public_key,
            content,
            created_at,
            kind,
            tags,
            signature,
            thread_id: None
        }
    }

    pub fn edit_content(&mut self, new_content: &str) {
        self.content = new_content.to_string();
    }

    pub fn is_empty(&self) -> bool {
        self.content.trim().is_empty()
    }
}

/*
impl From<Event> for Message {
    fn from(event: Event) -> Self {
        Self {
            id: event.id,
            public_key: event.public_key,
            content: event.content,
            created_at: event.created_at,
            kind: event.kind,
            tags: event.tags,
            signature: event.signature,
            thread_id: None
        }
    }
}
*/
