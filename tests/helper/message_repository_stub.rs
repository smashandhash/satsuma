use satsuma::{
    domain::message::{
        Message,
        MessageKind
    },
    infrastructure::message_repository::{
        MessageRepository,
        MessageRepositoryError
    }
};
use async_trait::async_trait;
use nostr_sdk::prelude::*;

pub struct MessageRepositoryStub {
    pub stubbed_error: Option<MessageRepositoryError>,
    pub messages: Vec<Message>,
}

impl MessageRepositoryStub {
    pub fn new(stubbed_error: Option<MessageRepositoryError>, messages: Vec<Message>) -> Self {
        Self { stubbed_error, messages }
    }
}

#[async_trait]
impl MessageRepository for MessageRepositoryStub {
    async fn send(
        &self,
        _sender_keys: &Keys,
        _content: &str,
        _kind: MessageKind,
    ) -> Result<Message, MessageRepositoryError> {
        self.stubbed_error.clone().map_or(Ok(self.make_message()), Err)
    }

    fn find_conversation(&self, sender_public_key: String, recipient_public_key: String) -> Vec<Message> {
        self.messages
            .iter()
            .filter( |m| {
                (m.public_key == sender_public_key) || (m.public_key == recipient_public_key)
            })
        .cloned()
            .collect()
    }
}

impl MessageRepositoryStub {
    fn make_message(&self) -> Message {
        Message::new("id".to_string(), "npub".to_string(), "Content".to_string(), 22u64, MessageKind::Direct("npub".to_string()))
    }
}
