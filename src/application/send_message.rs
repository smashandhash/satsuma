use crate::{
    domain::message::Message,
    domain::event_kind::EventKind
};

pub trait SendMessageUseCase {
    fn execute(&self, id: &str, sender_public_key: &str, content: &str, created_at: &u64, kind: &u32, tags: &Vec<Vec<String>>) -> Result<(), SendMessageUseCaseError>;
}

pub struct NostrSendMessageUseCase {
    pub max_length: usize,
}

impl SendMessageUseCase for NostrSendMessageUseCase {
    fn execute(&self, id: &str, sender_public_key: &str, content: &str, created_at: &u64, kind: &u32, tags: &Vec<Vec<String>>) -> Result<(), SendMessageUseCaseError> {
        let trimmed_content = content.trim();
        if trimmed_content.is_empty() {
            return Err(SendMessageUseCaseError::EmptyMessage);
        }

        if trimmed_content.chars().count() > self.max_length {
            return Err(SendMessageUseCaseError::MessageTooLong);
        }

        let event_kind = EventKind::get_event_kind(*kind)
            .map_err(|e| SendMessageUseCaseError::KindNotFound(e))?;

        let _message = Message::new(id.to_string(), sender_public_key.to_string(), trimmed_content.to_string(), *created_at, event_kind, tags.clone());
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum SendMessageUseCaseError {
    EmptyMessage,
    MessageTooLong,
    KindNotFound(String)
}
