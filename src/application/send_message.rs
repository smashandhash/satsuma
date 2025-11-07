use crate::{
    domain::{
        message::Message,
        event::Event
    },
    domain::services::nostr_event_validator::{
        NostrEventValidator,
        NostrEventValidatorError
    },
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

pub struct NostrSendMessageUseCase<V: NostrEventValidator, R: MessageRepository> {
    pub validator: V,
    pub repository: R
}

impl<V: NostrEventValidator, R: MessageRepository> NostrSendMessageUseCase<V, R> {
    pub const MAX_MESSAGE_LENGTH: usize = 2000;
}

#[async_trait]
impl<V: NostrEventValidator, R: MessageRepository> SendMessageUseCase for NostrSendMessageUseCase<V, R> where V: NostrEventValidator + Send + Sync, R: MessageRepository + Send + Sync {
    async fn execute(&self, message: &Message) -> Result<(), SendMessageUseCaseError> {
        self.validator.validate(&Event::from(message.clone())).map_err(|e| SendMessageUseCaseError::NostrError(e))?;

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
    NostrError(NostrEventValidatorError),
    RepositoryError(MessageRepositoryError)
}
