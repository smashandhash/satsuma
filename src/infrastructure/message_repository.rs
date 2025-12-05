use crate::domain::{
    chat_container::{
        ChatContainerContext,
        ChatContainerGroupType,
    },
    message::{
        Message,
        MessageKind
    },
};
use async_trait::async_trait;
use thiserror::Error;
use nostr_sdk::TagKind;
use nostr_sdk::prelude::*;
use std::{
    sync::Arc,
    str::FromStr,
    time::Duration,
};

#[async_trait]
pub trait MessageRepository {
    async fn send(
        &self,
        session_id: String,
        sender_keys: &Keys,
        content: &str,
        kind: MessageKind,
    ) -> Result<Message, MessageRepositoryError>;
    async fn find_root_messages(&self, container_id: String, context: ChatContainerContext, sender_public_key: String) -> Result<Vec<Message>, MessageRepositoryError>;
}

pub struct NostrMessageRepository {
    client: Arc<Client>,
}

impl NostrMessageRepository {
    pub async fn find_direct_root_messages(&self, other_public_key: String, sender_public_key: String) -> Result<Vec<Message>, MessageRepositoryError> {
        let filter = Filter::new()
            .kinds(vec![Kind::GiftWrap])
            .pubkeys(vec![
                PublicKey::from_str(&other_public_key).map_err(|e| MessageRepositoryError::UnknownError(e.to_string()))?,
                PublicKey::from_str(&sender_public_key).map_err(|e| MessageRepositoryError::UnknownError(e.to_string()))?
            ])
            .limit(100);

        let events = self.client.fetch_events(filter, Duration::from_secs(10)).await
            .map_err(|e| MessageRepositoryError::NetworkError(e.to_string()))?;

        let e_tag = SingleLetterTag::lowercase(Alphabet::E);

        // let mut result = Vec::new();

        for event in events {
            if event.kind == Kind::GiftWrap {
                let unwrapped = self.client.unwrap_gift_wrap(&event).await
                    .map_err(|e| MessageRepositoryError::UnknownError(e.to_string()))?;

                let is_thread = unwrapped.rumor.tags.iter().any(|t| matches!(t.kind(), TagKind::SingleLetter(sl) if sl == e_tag));

                if is_thread {
                    continue;
                }

                // result.push(Message::from_event(&unwrapped.rumor)); // TODO: Do it later
            }
        }

        // Ok(result)
        Ok(Vec::new())
    }

    pub async fn find_channel_root_messages(&self, _channel_id: String) -> Result<Vec<Message>, MessageRepositoryError> {
        Ok(Vec::new())
    }

    pub async fn find_group_root_messages(&self, _group_id: String) -> Result<Vec<Message>, MessageRepositoryError> {
        Ok(Vec::new())
    }
}

#[async_trait]
impl MessageRepository for NostrMessageRepository {
    async fn send(
        &self,
        session_id: String,
        sender_keys: &Keys,
        content: &str,
        kind: MessageKind,
    ) -> Result<Message, MessageRepositoryError> {
        let (event_kind, tags) = match kind {
            MessageKind::Direct => (
                Kind::EncryptedDirectMessage,
                Vec::new(),
            ),
            MessageKind::Thread(ref parent_id) => (
                Kind::TextNote,
                vec![Tag::event(EventId::from_hex(parent_id).map_err(|e| MessageRepositoryError::UnknownError(e.to_string()))?)],
            ),
            MessageKind::Group => (
                Kind::TextNote,
                vec![Tag::custom("group".into(), &[session_id.clone()])]
            ),
            MessageKind::Channel => (
                Kind::TextNote,
                vec![Tag::custom("channel".into(), &[session_id.clone()])]
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
            session_id,
            sender_keys.public_key().to_string(),
            event.content.clone(),
            event.created_at.as_secs(),
            kind);

        Ok(message)
    }

    async fn find_root_messages(&self, container_id: String, context: ChatContainerContext, sender_public_key: String) -> Result<Vec<Message>, MessageRepositoryError> {
        match context {
            ChatContainerContext::Direct { other_public_key } => {
                self.find_direct_root_messages(other_public_key, sender_public_key).await
            }   
            ChatContainerContext::Group { group_type, .. } => {
                match group_type {
                    ChatContainerGroupType::Private => self.find_group_root_messages(container_id).await,
                    ChatContainerGroupType::Channel => self.find_channel_root_messages(container_id).await,
                }
            }
        }
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
