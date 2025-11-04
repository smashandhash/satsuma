use crate::domain::{
    event::Event,
    services::{
        public_key_validator::{
            PublicKeyValidator,
            PublicKeyValidatorError
        },
        timestamp_validator::{
            TimestampValidator,
            TimestampValidatorError
        },
        event_id_validator::{
            EventIDValidator,
            EventIDValidatorError
        },
        kind_validator::{
            KindValidator,
            KindValidatorError
        },
        signature_verifier::{
            SignatureVerifier,
            SignatureVerifierError
        }
    }
};

pub trait NostrEventValidator {
    fn validate(&self, event: &Event) -> Result<(), NostrEventValidatorError>;
}

pub struct DefaultNostrEventValidator<PKV: PublicKeyValidator, TV: TimestampValidator, KV: KindValidator, EIV: EventIDValidator, SV: SignatureVerifier> {
    pub public_key_validator: PKV,
    pub timestamp_validator: TV,
    pub kind_validator: KV,
    pub event_id_validator: EIV,
    pub signature_verifier: SV
}

impl <PKV: PublicKeyValidator, TV: TimestampValidator, KV: KindValidator, EIV: EventIDValidator, SV: SignatureVerifier> NostrEventValidator for DefaultNostrEventValidator <PKV, TV, KV, EIV, SV> {
    fn validate(&self, event: &Event) -> Result<(), NostrEventValidatorError> {
        self.public_key_validator.validate_public_key(&event.public_key).map_err(|e| NostrEventValidatorError::PublicKeyError(e))?;
        self.timestamp_validator.validate_timestamp(event.created_at).map_err(|e| NostrEventValidatorError::TimestampError(e))?;
        self.kind_validator.validate_kind(event.kind).map_err(|e| NostrEventValidatorError::KindError(e))?;
        self.event_id_validator.validate_event_id(event).map_err(|e| NostrEventValidatorError::EventIDError(e))?;
        self.signature_verifier.verify_signature(&event.id, &event.public_key, &event.signature).map_err(|e| NostrEventValidatorError::SignatureError(e))?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NostrEventValidatorError {
    KindError(KindValidatorError),
    TimestampError(TimestampValidatorError),
    PublicKeyError(PublicKeyValidatorError),
    EventIDError(EventIDValidatorError),
    SignatureError(SignatureVerifierError)
}
