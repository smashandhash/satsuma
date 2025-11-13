use crate::{
    domain::message::{
        Message,
        MessageKind
    },
    infrastructure::{
        message_repository::{
            MessageRepository,
            MessageRepositoryError
        },
        local_storage::LocalStorage
    }
};
use async_trait::async_trait;

#[async_trait]
pub trait SendMessageUseCase {
    async fn execute(&self, 
        sender_public_key: String,
        content: String, 
        recipient_public_key: Option<String>, 
        parent_id: Option<String>,
        channel_id: Option<String>,
        group_id: Option<String>) -> Result<(), SendMessageUseCaseError>;
}

pub struct NostrSendMessageUseCase<R: MessageRepository, S: LocalStorage> {
    pub repository: R,
    pub storage: S
}

impl<R: MessageRepository, S: LocalStorage> NostrSendMessageUseCase<R, S> {
    pub const MAX_MESSAGE_LENGTH: usize = 2000;
}

#[async_trait]
impl<R: MessageRepository, S: LocalStorage> SendMessageUseCase for NostrSendMessageUseCase<R, S> where R: MessageRepository + Send + Sync, S: LocalStorage + Send + Sync {
    async fn execute(&self, 
        sender_public_key: String, 
        content: String, 
        recipient_public_key: Option<String>,
        parent_id: Option<String>,
        channel_id: Option<String>,
        group_id: Option<String>) -> Result<(), SendMessageUseCaseError> {
         let secret_key = self
            .storage
            .load_secret_key()
            .ok_or(SendMessageUseCaseError::Unauthorized)?;

        let keys = Keys::parse(&secret_key)
            .map_err(|e| SendMessageUseCaseError::InvalidKey(e.to_string()))?;

        let kind = if let Some(recipient) = &recipient_public_key {
            if parent_id.is_some() {
                MessageKind::Thread(parent_id.clone().unwrap())
            } else {
                MessageKind::Direct(recipient.clone())
            }
        } else if let Some(channel) = &channel_id {
            MessageKind::Channel(channel.clone())
        } else if let Some(group) = &group_id {
            MessageKind::Group(group.clone())
        } else {
            return Err(SendMessageUseCaseError::MissingDestination);
        };

        let trimmed_content = content.trim();
        if trimmed_content.is_empty() {
            return Err(SendMessageUseCaseError::EmptyMessage);
        }

        if trimmed_content.chars().count() > Self::MAX_MESSAGE_LENGTH {
            return Err(SendMessageUseCaseError::MessageTooLong);
        }

        let message = Message::new(
            "".to_string(),
            sender_public_key,
            trimmed_content.to_string(),
            chrono::Utc::now().timestamp() as u64,
            kind.clone());

        self.repository.send(&message).await.map_err(|e| SendMessageUseCaseError::RepositoryError(e))
    }
}

#[derive(Debug, PartialEq)]
pub enum SendMessageUseCaseError {
    MissingDestination,
    EmptyMessage,
    MessageTooLong,
    RepositoryError(MessageRepositoryError)
}
