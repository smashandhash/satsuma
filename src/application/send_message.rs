use crate::{
    domain::message::{
        Message,
        MessageKind
    },
    infrastructure::{
        key_provider::{
            KeyProvider,
            KeyProviderError
        },
        message_repository::{
            MessageRepository,
            MessageRepositoryError
        },
        local_storage::LocalStorage
    }
};
use async_trait::async_trait;
use nostr_sdk::prelude::*;

#[async_trait]
pub trait SendMessageUseCase {
    async fn execute(&self, 
        content: String, 
        recipient_public_key: Option<String>, 
        parent_id: Option<String>,
        channel_id: Option<String>,
        group_id: Option<String>) -> Result<Message, SendMessageUseCaseError>;
}

pub struct NostrSendMessageUseCase<K: KeyProvider, R: MessageRepository, S: LocalStorage> {
    pub provider: K,
    pub repository: R,
    pub storage: S
}

impl<K: KeyProvider, R: MessageRepository, S: LocalStorage> NostrSendMessageUseCase<K, R, S> {
    pub const MAX_MESSAGE_LENGTH: usize = 2000;
}

#[async_trait]
impl<K: KeyProvider, R: MessageRepository, S: LocalStorage> SendMessageUseCase for NostrSendMessageUseCase<K, R, S> where K: KeyProvider + Send + Sync, R: MessageRepository + Send + Sync, S: LocalStorage + Send + Sync {
    async fn execute(&self, 
        content: String, 
        recipient_public_key: Option<String>,
        parent_id: Option<String>,
        channel_id: Option<String>,
        group_id: Option<String>) -> Result<Message, SendMessageUseCaseError> {
         let secret_key = self
            .storage
            .load_secret_key()
            .map_err(|e| SendMessageUseCaseError::Unauthorized(e.to_string()))?;

        let keys = self.provider.parse_secret_key(&secret_key)
            .await
            .map_err(|e| SendMessageUseCaseError::InvalidKey(e))?;

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

        self.repository.send(&keys, trimmed_content, kind).await.map_err(|e| SendMessageUseCaseError::RepositoryError(e))
    }
}

#[derive(Debug, PartialEq)]
pub enum SendMessageUseCaseError {
    Unauthorized(String),
    InvalidKey(KeyProviderError),
    MissingDestination,
    EmptyMessage,
    MessageTooLong,
    RepositoryError(MessageRepositoryError)
}
