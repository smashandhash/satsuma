use crate::{
    domain::message::Message,
    infrastructure::{
        local_storage::LocalStorage,
        key_provider::{
            KeyProvider,
            KeyProviderError,
        },
        chat_container_repository::{
            ChatContainerRepository,
            ChatContainerRepositoryError,
        },
        message_repository::{
            MessageRepository,
            MessageRepositoryError,
        },
    },
};
use async_trait::async_trait;
use std::sync::Arc;
use nostr_sdk::prelude::*;

#[async_trait]
pub trait LoadMessagesUseCase {
    async fn execute(&self, chat_container_id: String) -> Result<Vec<Message>, LoadMessagesUseCaseError>;
}

pub struct LoadMessagesUseCaseImplementation<S: LocalStorage, K: KeyProvider, CR: ChatContainerRepository, MR: MessageRepository> {
    pub storage: Arc<S>,
    pub provider: Arc<K>,
    pub container_repository: Arc<CR>,
    pub message_repository: Arc<MR>
}

impl<S: LocalStorage, K: KeyProvider, CR: ChatContainerRepository, MR: MessageRepository> LoadMessagesUseCaseImplementation<S, K, CR, MR> {
    pub fn new(storage: Arc<S>, provider: Arc<K>, container_repository: Arc<CR>, message_repository: Arc<MR>) -> Self {
        Self {
            storage,
            provider,
            container_repository,
            message_repository,
        }
    }
}

#[async_trait]
impl<S: LocalStorage, K: KeyProvider, CR: ChatContainerRepository, MR: MessageRepository> LoadMessagesUseCase for LoadMessagesUseCaseImplementation<S, K, CR, MR> where S: LocalStorage + Send + Sync, K: KeyProvider + Send + Sync, CR: ChatContainerRepository + Send + Sync, MR: MessageRepository + Send + Sync {
    async fn execute(&self, chat_container_id: String) -> Result<Vec<Message>, LoadMessagesUseCaseError> {
        let secret_key = self
            .storage
            .load_secret_key()
            .map_err(|e| LoadMessagesUseCaseError::Unauthorized(e.to_string()))?;

        let keys = self.provider.parse_secret_key(&secret_key)
            .await
            .map_err(|e| LoadMessagesUseCaseError::InvalidKey(e))?;

        let chat_container = self.container_repository.load(chat_container_id.clone()).map_err(|e| LoadMessagesUseCaseError::ContainerRepositoryError(e))?;

        let messages = self.message_repository.find_root_messages(chat_container_id.clone(), chat_container.context, keys.public_key().to_bech32().unwrap()).await.map_err(|e| LoadMessagesUseCaseError::MessageRepositoryError(e))?;

        Ok(messages.clone())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoadMessagesUseCaseError {
    Unauthorized(String),
    InvalidKey(KeyProviderError),
    ContainerRepositoryError(ChatContainerRepositoryError),
    MessageRepositoryError(MessageRepositoryError),
}
