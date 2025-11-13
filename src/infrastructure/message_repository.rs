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
    async fn send(
        &self,
        sender_keys: &Keys,
        content: &str,
        kind: MessageKind,
    ) -> Result<Message, MessageRepositoryError>;
    fn find_conversation(&self, sender_public_key: String, recipient_public_key: String) -> Vec<Message>;
}

pub struct NostrMessageRepository {
    client: Arc<Client>,
}

#[async_trait]
impl MessageRepository for NostrMessageRepository {
    async fn send(
        &self,
        sender_keys: &Keys,
        content: &str,
        kind: MessageKind,
    ) -> Result<Message, MessageRepositoryError> {
        let (event_kind, tags) = match kind {
            MessageKind::Direct(ref recipient_pubkey) => (
                Kind::EncryptedDirectMessage,
                vec![Tag::public_key(Keys::parse(recipient_pubkey).unwrap().public_key())],
            ),
            MessageKind::Thread(ref parent_id) => (
                Kind::TextNote,
                vec![Tag::event(EventId::from_hex(parent_id).map_err(|e| MessageRepositoryError::UnknownError(e.to_string()))?)],
            ),
            MessageKind::Group(ref group_id) => (
                Kind::TextNote,
                vec![Tag::custom("group".into(), &[group_id.clone()])]
            ),
            MessageKind::Channel(ref channel_id) => (
                Kind::TextNote,
                vec![Tag::custom("channel".into(), &[channel_id.clone()])]
            ),
        };

        let mut event_builder = EventBuilder::new(event_kind, content);
        event_builder = event_builder.tags(tags);

        let event = event_builder.sign(sender_keys).await.map_err(|e| MessageRepositoryError::UnknownError(e.to_string()))?;

        self.client
            .send_event(&event.clone())
            .await
            .map_err(|e| MessageRepositoryError::PublishError(e.to_string()))?;

        let message = Message::new(
            event.id.to_hex(),
            sender_keys.public_key().to_string(),
            event.content.clone(),
            event.created_at.as_secs(),
            kind);

        Ok(message)
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
    UnsupportedMessageKind,

    #[error("Publish error: {0}")]
    PublishError(String)
}
