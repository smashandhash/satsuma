use satsuma::{
    domain::message::Message,
    infrastructure::message_repository::{
        MessageRepository,
        MessageRepositoryError
    }
};
use async_trait::async_trait;

pub struct MessageRepositoryStub {
    messages: Vec<Message>,
}

impl MessageRepositoryStub {
    pub fn new(messages: Vec<Message>) -> Self {
        Self { messages }
    }
}

#[async_trait]
impl MessageRepository for MessageRepositoryStub {
    async fn send(&self, _message: &Message) -> Result<(), MessageRepositoryError> {
        Ok(())
    }

    fn find_conversation(&self, sender_public_key: String, recipient_public_key: String) -> Vec<Message> {
        self.messages
            .iter()
            .filter( |m| {
                (m.public_key == sender_public_key) || (m.public_key == recipient_public_key)
            })
        .cloned()
            .collect()
    }
}
