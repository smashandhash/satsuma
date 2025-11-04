use crate::{
    domain::{
        message::Message,
        event::Event
    },
    domain::services::nostr_event_validator::{
        NostrEventValidator,
        NostrEventValidatorError
    }
};

pub trait SendMessageUseCase {
    fn execute(&self, message: Message) -> Result<(), SendMessageUseCaseError>;
}

pub struct NostrSendMessageUseCase<V: NostrEventValidator> {
    pub max_length: usize,
    pub validator: V
}

impl<V: NostrEventValidator> SendMessageUseCase for NostrSendMessageUseCase<V> {
    fn execute(&self, message: Message) -> Result<(), SendMessageUseCaseError> {
        self.validator.validate(&Event::from(message.clone())).map_err(|e| SendMessageUseCaseError::NostrError(e))?;

        let trimmed_content = message.content.trim();
        if trimmed_content.is_empty() {
            return Err(SendMessageUseCaseError::EmptyMessage);
        }

        if trimmed_content.chars().count() > self.max_length {
            return Err(SendMessageUseCaseError::MessageTooLong);
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum SendMessageUseCaseError {
    EmptyMessage,
    MessageTooLong,
    NostrError(NostrEventValidatorError),
}
