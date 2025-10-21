use crate::{
    domain::{
        message::Message,
        event::Event
    },
    domain::services::validate_timestamp::{
        validate_timestamp,
        ValidateTimestampError
    },
    domain::services::validate_public_key::{
        validate_public_key,
        ValidatePublicKeyError
    },
    domain::services::validate_event_id::{
        validate_event_id,
        ValidateEventIDError
    },
    domain::services::validate_kind::{
        validate_kind,
        ValidateKindError
    },
    domain::services::verify_signature::{
        verify_signature,
        VerifySignatureError
    }
};

pub trait SendMessageUseCase {
    fn execute(&self, message: Message) -> Result<(), SendMessageUseCaseError>;
}

pub struct NostrSendMessageUseCase {
    pub max_length: usize,
}

impl SendMessageUseCase for NostrSendMessageUseCase {
    fn execute(&self, message: Message) -> Result<(), SendMessageUseCaseError> {
        validate_public_key(&message.public_key).map_err(|e| SendMessageUseCaseError::PublicKeyError(e))?;
        validate_event_id(&Event::from(message.clone())).map_err(|e| SendMessageUseCaseError::EventIDError(e))?;
        verify_signature(&message.id, &message.public_key, &message.signature).map_err(|e| SendMessageUseCaseError::SignatureError(e))?;
        validate_timestamp(message.created_at).map_err(|e| SendMessageUseCaseError::TimestampError(e))?;

        let trimmed_content = message.content.trim();
        if trimmed_content.is_empty() {
            return Err(SendMessageUseCaseError::EmptyMessage);
        }

        if trimmed_content.chars().count() > self.max_length {
            return Err(SendMessageUseCaseError::MessageTooLong);
        }

        validate_kind(message.kind).map_err(|e| SendMessageUseCaseError::KindError(e))?;

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum SendMessageUseCaseError {
    EmptyMessage,
    MessageTooLong,
    KindError(ValidateKindError),
    TimestampError(ValidateTimestampError),
    PublicKeyError(ValidatePublicKeyError),
    EventIDError(ValidateEventIDError),
    SignatureError(VerifySignatureError)
}
