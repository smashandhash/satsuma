use crate::{
    domain::message::Message,
    infrastructure::message_repository::{
        MessageRepository,
        MessageRepositoryError
    }
};
use async_trait::async_trait;

#[async_trait]
pub trait SendMessageUseCase {
    async fn execute(&self, message: &Message) -> Result<(), SendMessageUseCaseError>;
}

pub struct NostrSendMessageUseCase<R: MessageRepository> {
    pub repository: R
}

impl<R: MessageRepository> NostrSendMessageUseCase<R> {
    pub const MAX_MESSAGE_LENGTH: usize = 2000;
}

#[async_trait]
impl<R: MessageRepository> SendMessageUseCase for NostrSendMessageUseCase<R> where R: MessageRepository + Send + Sync {
    async fn execute(&self, message: &Message) -> Result<(), SendMessageUseCaseError> {
        let trimmed_content = message.content.trim();
        if trimmed_content.is_empty() {
            return Err(SendMessageUseCaseError::EmptyMessage);
        }

        if trimmed_content.chars().count() > Self::MAX_MESSAGE_LENGTH {
            return Err(SendMessageUseCaseError::MessageTooLong);
        }

        self.repository.send(message).await.map_err(|e| SendMessageUseCaseError::RepositoryError(e))
    }
}

#[derive(Debug, PartialEq)]
pub enum SendMessageUseCaseError {
    EmptyMessage,
    MessageTooLong,
    RepositoryError(MessageRepositoryError)
}
