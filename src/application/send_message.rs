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
    domain::services::event_id_validator::{
        EventIDValidator,
        EventIDValidatorError
    },
    domain::services::kind_validator::{
        KindValidator,
        KindValidatorError
    },
    domain::services::signature_verifier::{
        SignatureVerifier,
        SignatureVerifierError
    }
};

pub trait SendMessageUseCase {
    fn execute(&self, message: Message) -> Result<(), SendMessageUseCaseError>;
}

pub struct NostrSendMessageUseCase<KV: KindValidator, EIV: EventIDValidator, SV: SignatureVerifier> {
    pub max_length: usize,
    pub kind_validator: KV,
    pub event_id_validator: EIV,
    pub signature_verifier: SV
}

impl<KV: KindValidator, EIV: EventIDValidator, SV: SignatureVerifier> SendMessageUseCase for NostrSendMessageUseCase<KV, EIV, SV> {
    fn execute(&self, message: Message) -> Result<(), SendMessageUseCaseError> {
        validate_public_key(&message.public_key).map_err(|e| SendMessageUseCaseError::PublicKeyError(e))?;
        self.kind_validator.validate_kind(message.kind).map_err(|e| SendMessageUseCaseError::KindError(e))?;
        self.event_id_validator.validate_event_id(&Event::from(message.clone())).map_err(|e| SendMessageUseCaseError::EventIDError(e))?;
        self.signature_verifier.verify_signature(&message.id, &message.public_key, &message.signature).map_err(|e| SendMessageUseCaseError::SignatureError(e))?;
        validate_timestamp(message.created_at).map_err(|e| SendMessageUseCaseError::TimestampError(e))?;

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
    KindError(KindValidatorError),
    TimestampError(ValidateTimestampError),
    PublicKeyError(ValidatePublicKeyError),
    EventIDError(EventIDValidatorError),
    SignatureError(SignatureVerifierError)
}
