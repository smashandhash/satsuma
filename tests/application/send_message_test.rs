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
            validate_public_key::ValidatePublicKeyError,
            kind_validator::KindValidatorError,
            event_id_validator::EventIDValidatorError,
            signature_verifier::SignatureVerifierError
        }
    };
    use crate::helper::{
        generate_event_id::generate_event_id,
        kind_validator_stub::KindValidatorStub,
        event_id_validator_stub::EventIDValidatorStub,
        signature_verifier_stub::SignatureVerifierStub
    };
    use chrono::{Utc, Duration};
    use rstest::rstest;

    const VALID_PUBKEY: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    #[rstest]
    #[case("send message to another user", 200, &VALID_PUBKEY, "Hello, Bob!", Utc::now().timestamp() as u64, None, None, None, Ok(()))]
    #[case("rejected for empty message", 200, &VALID_PUBKEY, "", Utc::now().timestamp() as u64, None, None, None, Err(SendMessageUseCaseError::EmptyMessage))]
    #[case("rejected for message has only spaces", 200, &VALID_PUBKEY, "   ", Utc::now().timestamp() as u64, None, None, None, Err(SendMessageUseCaseError::EmptyMessage))]
    #[case("rejected for message is too long", 8, &VALID_PUBKEY, "Hello, Bob", Utc::now().timestamp() as u64, None, None, None, Err(SendMessageUseCaseError::MessageTooLong))]
    #[case("rejected for message's kind not found", 200, &VALID_PUBKEY, "Hello, Bob", Utc::now().timestamp() as u64, Some(KindValidatorError::InvalidKindValue(5000)), None, None, Err(SendMessageUseCaseError::KindError(KindValidatorError::InvalidKindValue(5000))))]
    #[case("rejected for timestamp is invalid", 200, &VALID_PUBKEY, "Hello, Bob", u64::MAX, None, None, None, Err(SendMessageUseCaseError::TimestampError(ValidateTimestampError::InvalidTimestamp)))]
    #[case("rejected for timestamp is far in the future", 200, &VALID_PUBKEY, "Hello, Bob", (Utc::now() + Duration::seconds(60 * 10)).timestamp() as u64, None, None, None, Err(SendMessageUseCaseError::TimestampError(ValidateTimestampError::TimestampTooFarInTheFuture)))]
    #[case("rejected for timestamp is too old", 200, &VALID_PUBKEY, "Hello, Bob", (Utc::now() - Duration::days(8)).timestamp() as u64, None, None, None, Err(SendMessageUseCaseError::TimestampError(ValidateTimestampError::TimestampTooOld)))]
    #[case("rejected for invalid public key's length", 200, "npub100", "Hello", Utc::now().timestamp() as u64, None, None, None, Err(SendMessageUseCaseError::PublicKeyError(ValidatePublicKeyError::InvalidPublicKeyLength)))]
    #[case("rejected for public key isn't hex-encoded", 200, "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz", "Hello", Utc::now().timestamp() as u64, None, None, None, Err(SendMessageUseCaseError::PublicKeyError(ValidatePublicKeyError::PublicKeyNotHexEncoded)))]
    #[case("rejected for invalid ID hex", 200, &VALID_PUBKEY, "Hello, Bob", Utc::now().timestamp() as u64, None, None, Some(SignatureVerifierError::InvalidIDHex), Err(SendMessageUseCaseError::SignatureError(SignatureVerifierError::InvalidIDHex)))]
    #[case("rejected for ID isn't equal to our generated ID", 200, &VALID_PUBKEY, "Hello, Bob", Utc::now().timestamp() as u64, None, Some(EventIDValidatorError::EventIDMismatch), None, Err(SendMessageUseCaseError::EventIDError(EventIDValidatorError::EventIDMismatch)))]
    fn send_message(
        #[case] _label: &str,
        #[case] max_length: usize,
        #[case] public_key: &str,
        #[case] content: &str,
        #[case] created_at: u64,
        #[case] kind_simulated_error: Option<KindValidatorError>,
        #[case] event_id_simulated_error: Option<EventIDValidatorError>,
        #[case] signature_simulated_error: Option<SignatureVerifierError>,
        #[case] expected: Result<(), SendMessageUseCaseError>) {
        let id = generate_event_id(public_key, created_at.clone(), 0, &Vec::new(), content);
        let kind_validator = KindValidatorStub { simulated_error: kind_simulated_error };
        let event_id_validator = EventIDValidatorStub { simulated_error: event_id_simulated_error };
        let signature_verifier = SignatureVerifierStub { simulated_error: signature_simulated_error };
        let use_case = NostrSendMessageUseCase { max_length, kind_validator, event_id_validator, signature_verifier };
        let message = Message::new(id, public_key.to_string(), content.to_string(), created_at, 0, Vec::new(), "".to_string());
        let result = use_case.execute(message);

        assert_eq!(result, expected);
    }
}
