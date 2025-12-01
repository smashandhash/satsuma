use crate::{
    domain::{
        chat_container::{
            ChatContainerContext,
            ChatContainerGroupType,
        },
        message::{
            Message,
            MessageKind
        },
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
use std::sync::Arc;
use nostr_sdk::prelude::*;

#[async_trait]
pub trait SendMessageUseCase {
    async fn execute(&self, 
        content: String,
        session_id: String,
        chat_container_context: ChatContainerContext,
        parent_id: Option<String>) -> Result<Message, SendMessageUseCaseError>;
}

pub struct NostrSendMessageUseCase<K: KeyProvider, R: MessageRepository, S: LocalStorage> {
    pub provider: Arc<K>,
    pub repository: Arc<R>,
    pub storage: Arc<S>
}

impl<K: KeyProvider, R: MessageRepository, S: LocalStorage> NostrSendMessageUseCase<K, R, S> {
    pub const MAX_MESSAGE_LENGTH: usize = 2000;
}

#[async_trait]
impl<K: KeyProvider, R: MessageRepository, S: LocalStorage> SendMessageUseCase for NostrSendMessageUseCase<K, R, S> where K: KeyProvider + Send + Sync, R: MessageRepository + Send + Sync, S: LocalStorage + Send + Sync {
    async fn execute(&self, 
        content: String,
        session_id: String,
        chat_container_context: ChatContainerContext,
        parent_id: Option<String>) -> Result<Message, SendMessageUseCaseError> {
         let secret_key = self
            .storage
            .load_secret_key()
            .map_err(|e| SendMessageUseCaseError::Unauthorized(e.to_string()))?;

        let keys = self.provider.parse_secret_key(&secret_key)
            .await
            .map_err(|e| SendMessageUseCaseError::InvalidKey(e))?;

        let kind = match chat_container_context {
            ChatContainerContext::Direct { .. } => parent_id
                .map(MessageKind::Thread)
                .unwrap_or(MessageKind::Direct),

            ChatContainerContext::Group { group_type, ..} => parent_id.map(MessageKind::Thread).unwrap_or(
                match group_type {
                    ChatContainerGroupType::Private => MessageKind::Group,
                    ChatContainerGroupType::Channel => MessageKind::Channel
                }
            ),
        };

        let trimmed_content = content.trim();
        if trimmed_content.is_empty() {
            return Err(SendMessageUseCaseError::EmptyMessage);
        }

        if trimmed_content.chars().count() > Self::MAX_MESSAGE_LENGTH {
            return Err(SendMessageUseCaseError::MessageTooLong);
        }

        self.repository.send(session_id, &keys, trimmed_content, kind).await.map_err(|e| SendMessageUseCaseError::RepositoryError(e))
    }
}

#[derive(Debug, PartialEq)]
pub enum SendMessageUseCaseError {
    Unauthorized(String),
    InvalidKey(KeyProviderError),
    EmptyMessage,
    MessageTooLong,
    RepositoryError(MessageRepositoryError)
}
