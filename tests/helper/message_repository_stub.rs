use satsuma::{
    domain::{
        message::{
            Message,
            MessageKind
        },
        chat_container::ChatContainerContext,
    },
    infrastructure::message_repository::{
        MessageRepository,
        MessageRepositoryError
    }
};
use async_trait::async_trait;
use nostr_sdk::prelude::*;

pub struct MessageRepositoryStub {
    pub simulated_error: Option<MessageRepositoryError>,
}

impl MessageRepositoryStub {
    pub fn new(simulated_error: Option<MessageRepositoryError>) -> Self {
        Self { simulated_error }
    }
}

#[async_trait]
impl MessageRepository for MessageRepositoryStub {
    async fn send(
        &self,
        _session_id: String,
        _sender_keys: &Keys,
        _content: &str,
        _kind: MessageKind,
    ) -> Result<Message, MessageRepositoryError> {
        self.simulated_error.clone().map_or(Ok(self.make_message()), Err)
    }

    async fn find_root_messages(&self, _container_id: String, _context: ChatContainerContext, _sender_public_key: String) -> Result<Vec<Message>, MessageRepositoryError> {
        if let Some(err) = &self.simulated_error {
            Err(err.clone())
        } else {
            Ok(vec![self.make_message()])
        }

    }
}

impl MessageRepositoryStub {
    fn make_message(&self) -> Message {
        Message::new("id".to_string(), "session_id".to_string(), "public_key".to_string(), "content".to_string(), 22u64, MessageKind::Direct)
    }
}
