use crate::domain::message::Message;
use async_trait::async_trait;

#[async_trait]
pub trait MessageRepository {
    async fn send(&self, message: &Message) -> Result<(), MessageRepositoryError>;
    fn find_conversation(&self, sender_public_key: String, recipient_public_key: String) -> Vec<Message>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum MessageRepositoryError {
    NetworkError(String),
    UnknownError(String)
}
