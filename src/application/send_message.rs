use crate::{
    domain::user::User,
    domain::message::Message,
    domain::event_kind::EventKind
};

pub trait SendMessageUseCase {
    fn execute(&self, sender: &User, content: &str) -> Result<Message, SendMessageUseCaseError>;
}

pub struct NostrSendMessageUseCase {
    pub max_length: usize,
}

impl SendMessageUseCase for NostrSendMessageUseCase {
    fn execute(&self, sender: &User, content: &str) -> Result<Message, SendMessageUseCaseError> {
        let trimmed_content = content.trim();
        if trimmed_content.is_empty() {
            return Err(SendMessageUseCaseError::EmptyMessage);
        }

        if trimmed_content.chars().count() > self.max_length {
            return Err(SendMessageUseCaseError::MessageTooLong);
        }

        let message = Message::new(&sender.public_key, trimmed_content, EventKind::PrivateOrGroupMessage, Vec::new());
        Ok(message)
    }
}

#[derive(Debug, PartialEq)]
pub enum SendMessageUseCaseError {
    EmptyMessage,
    MessageTooLong
}
