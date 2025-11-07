use crate::domain::message::Message;
use async_trait::async_trait;
use thiserror::Error;

#[async_trait]
pub trait MessageRepository {
    async fn send(&self, message: &Message) -> Result<(), MessageRepositoryError>;
    fn find_conversation(&self, sender_public_key: String, recipient_public_key: String) -> Vec<Message>;
}

#[derive(Error, Clone, Debug, PartialEq)]
pub enum MessageRepositoryError {
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Unknown error: {0}")]
    UnknownError(String)
}
