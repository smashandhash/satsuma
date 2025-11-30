#[derive(Debug, Clone, PartialEq)]
pub struct ChatSession {
    pub id: String,
    pub container_id: String,
    pub context: ChatSessionContext,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChatSessionContext {
    Root,
    Thread { parent_message_id: String },
}

impl ChatSession {
    pub fn new(id: String, container_id: String, context: ChatSessionContext) -> Self {
        Self {
            id,
            container_id,
            context,
        }
    }
}
