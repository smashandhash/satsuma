#[cfg(test)]
mod tests {
    use satsuma::{
        application::send_message::{
            SendMessageUseCase,
            SendMessageUseCaseError,
            NostrSendMessageUseCase,
        },
        domain::message::Message,
        domain::services::{
            validate_timestamp::ValidateTimestampError,
            public_key_validator::PublicKeyValidatorError,
            kind_validator::KindValidatorError,
            event_id_validator::EventIDValidatorError,
            signature_verifier::SignatureVerifierError
        }
    };
    use crate::helper::{
        generate_event_id::generate_event_id,
        public_key_validator_stub::PublicKeyValidatorStub,
        kind_validator_stub::KindValidatorStub,
        event_id_validator_stub::EventIDValidatorStub,
        signature_verifier_stub::SignatureVerifierStub
    };
    use chrono::{Utc, Duration};
    use rstest::rstest;

    #[rstest]
    #[case("send message to another user", 200, "Hello, Bob!", Utc::now().timestamp() as u64, None, None, None, None, Ok(()))]
    #[case("rejected for empty message", 200, "", Utc::now().timestamp() as u64, None, None, None, None, Err(SendMessageUseCaseError::EmptyMessage))]
    #[case("rejected for message has only spaces", 200, "   ", Utc::now().timestamp() as u64, None, None, None, None, Err(SendMessageUseCaseError::EmptyMessage))]
    #[case("rejected for message is too long", 8, "Hello, Bob", Utc::now().timestamp() as u64, None, None, None, None, Err(SendMessageUseCaseError::MessageTooLong))]
    #[case("rejected for message's kind not found", 200, "Hello, Bob", Utc::now().timestamp() as u64, None, Some(KindValidatorError::InvalidKindValue(5000)), None, None, Err(SendMessageUseCaseError::KindError(KindValidatorError::InvalidKindValue(5000))))]
    #[case("rejected for timestamp is invalid", 200, "Hello, Bob", u64::MAX, None, None, None, None, Err(SendMessageUseCaseError::TimestampError(ValidateTimestampError::InvalidTimestamp)))]
    #[case("rejected for timestamp is far in the future", 200, "Hello, Bob", (Utc::now() + Duration::seconds(60 * 10)).timestamp() as u64, None, None, None, None, Err(SendMessageUseCaseError::TimestampError(ValidateTimestampError::TimestampTooFarInTheFuture)))]
    #[case("rejected for timestamp is too old", 200, "Hello, Bob", (Utc::now() - Duration::days(8)).timestamp() as u64, None, None, None, None, Err(SendMessageUseCaseError::TimestampError(ValidateTimestampError::TimestampTooOld)))]
    #[case("rejected for invalid public key's length", 200, "Hello", Utc::now().timestamp() as u64, Some(PublicKeyValidatorError::InvalidPublicKeyLength), None, None, None, Err(SendMessageUseCaseError::PublicKeyError(PublicKeyValidatorError::InvalidPublicKeyLength)))]
    #[case("rejected for public key isn't hex-encoded", 200, "Hello", Utc::now().timestamp() as u64, Some(PublicKeyValidatorError::PublicKeyNotHexEncoded), None, None, None, Err(SendMessageUseCaseError::PublicKeyError(PublicKeyValidatorError::PublicKeyNotHexEncoded)))]
    #[case("rejected for invalid ID hex", 200, "Hello, Bob", Utc::now().timestamp() as u64, None, None, None, Some(SignatureVerifierError::InvalidIDHex), Err(SendMessageUseCaseError::SignatureError(SignatureVerifierError::InvalidIDHex)))]
    #[case("rejected for ID isn't equal to our generated ID", 200, "Hello, Bob", Utc::now().timestamp() as u64, None, None, Some(EventIDValidatorError::EventIDMismatch), None, Err(SendMessageUseCaseError::EventIDError(EventIDValidatorError::EventIDMismatch)))]
    fn send_message(
        #[case] _label: &str,
        #[case] max_length: usize,
        #[case] content: &str,
        #[case] created_at: u64,
        #[case] public_key_simulated_error: Option<PublicKeyValidatorError>,
        #[case] kind_simulated_error: Option<KindValidatorError>,
        #[case] event_id_simulated_error: Option<EventIDValidatorError>,
        #[case] signature_simulated_error: Option<SignatureVerifierError>,
        #[case] expected: Result<(), SendMessageUseCaseError>) {
        let id = generate_event_id("", created_at.clone(), 0, &Vec::new(), content);
        let public_key_validator = PublicKeyValidatorStub { simulated_error: public_key_simulated_error };
        let kind_validator = KindValidatorStub { simulated_error: kind_simulated_error };
        let event_id_validator = EventIDValidatorStub { simulated_error: event_id_simulated_error };
        let signature_verifier = SignatureVerifierStub { simulated_error: signature_simulated_error };
        let use_case = NostrSendMessageUseCase { max_length, public_key_validator, kind_validator, event_id_validator, signature_verifier };
        let message = Message::new(id, "".to_string(), content.to_string(), created_at, 0, Vec::new(), "".to_string());
        let result = use_case.execute(message);

        assert_eq!(result, expected);
    }
}
