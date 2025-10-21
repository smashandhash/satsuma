use super::message::Message;

pub struct Event {
    pub id: String,
    pub public_key: String,
    pub created_at: u64,
    pub kind: u32,
    pub tags: Vec<Vec<String>>,
    pub content: String,
    pub signature: String
}

impl From<Message> for Event {
    fn from(message: Message) -> Self {
        Self {
            id: message.id,
            public_key: message.public_key,
            created_at: message.created_at,
            kind: message.kind,
            tags: message.tags,
            content: message.content,
            signature: message.signature
        }
    }
}
