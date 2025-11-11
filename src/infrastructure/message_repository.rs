use crate::domain::message::{
    Message,
    MessageKind
};
use async_trait::async_trait;
use thiserror::Error;
use nostr_sdk::prelude::*;
use std::sync::Arc;

#[async_trait]
pub trait MessageRepository {
    async fn send(&self, message: &Message) -> Result<(), MessageRepositoryError>;
    fn find_conversation(&self, sender_public_key: String, recipient_public_key: String) -> Vec<Message>;
}

pub struct NostrMessageRepository {
    client: Arc<Client>,
}

#[async_trait]
impl MessageRepository for NostrMessageRepository {
    async fn send(&self, message: &Message) -> Result<(), MessageRepositoryError> {
        match &message.kind {
            MessageKind::Direct(recipient_pubkey_str) => {
                let recipient_public_key = Keys::parse(recipient_pubkey_str)
                    .map_err(|e| MessageRepositoryError::UnknownError(e.to_string()))?.public_key();

                let _ = self.client.as_ref().send_private_msg(recipient_public_key, &message.content, []).await.map_err(|e| MessageRepositoryError::UnknownError(e.to_string()));

                Ok(())
            }

            _ => Err(MessageRepositoryError::UnsupportedMessageKind)
        }
    }

    fn find_conversation(&self, _sender_public_key: String, _recipient_public_key: String) -> Vec<Message> {
        Vec::new()
    }
}

#[derive(Error, Clone, Debug, PartialEq)]
pub enum MessageRepositoryError {
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Unknown error: {0}")]
    UnknownError(String),

    #[error("UnsupportedMessageKind")]
    UnsupportedMessageKind
}
