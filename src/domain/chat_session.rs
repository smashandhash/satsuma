use crate::domain::message::Message;

#[derive(Debug, Clone, PartialEq)]
pub struct ChatSession {
    pub id: String,
    pub context: ChatSessionContext,
    pub messages: Vec<Message>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChatSessionContext {
    Direct(String),
    Group(String),
    Channel(String),
    Thread(String),
}

impl ChatSession {
    pub fn new(id: String, context: ChatSessionContext) -> Self {
        Self {
            id,
            context,
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }
}
