#[cfg(test)]
mod tests {
    use satsuma::{
        application::send_message::SendMessageUseCase,
        application::send_message::SendMessageUseCaseError,
        application::send_message::NostrSendMessageUseCase,
        domain::message::Message,
        domain::services::{
            validate_timestamp::ValidateTimestampError,
            validate_public_key::ValidatePublicKeyError,
            validate_kind::ValidateKindError,
            validate_event_id::ValidateEventIDError
        }
    };
    use crate::helper::generate_event_id::generate_event_id;
    use chrono::{Utc, Duration};
    use rstest::rstest;

    const VALID_PUBKEY: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    #[rstest]
    #[case("send message to another user", 200, &VALID_PUBKEY, "Hello, Bob!", 14, Utc::now().timestamp() as u64, None, Ok(()))]
    #[case("rejected for empty message", 200, &VALID_PUBKEY, "", 14, Utc::now().timestamp() as u64, None, Err(SendMessageUseCaseError::EmptyMessage))]
    #[case("rejected for message has only spaces", 200, &VALID_PUBKEY, "   ", 14, Utc::now().timestamp() as u64, None, Err(SendMessageUseCaseError::EmptyMessage))]
    #[case("rejected for message is too long", 8, &VALID_PUBKEY, "Hello, Bob", 14, Utc::now().timestamp() as u64, None, Err(SendMessageUseCaseError::MessageTooLong))]
    #[case("rejected for message's kind not found", 200, &VALID_PUBKEY, "Hello, Bob", 5000, Utc::now().timestamp() as u64, None, Err(SendMessageUseCaseError::KindError(ValidateKindError::InvalidKindValue(5000))))]
    #[case("rejected for timestamp is invalid", 200, &VALID_PUBKEY, "Hello, Bob", 14, u64::MAX, None, Err(SendMessageUseCaseError::TimestampError(ValidateTimestampError::InvalidTimestamp)))]
    #[case("rejected for timestamp is far in the future", 200, &VALID_PUBKEY, "Hello, Bob", 14, (Utc::now() + Duration::seconds(60 * 10)).timestamp() as u64, None, Err(SendMessageUseCaseError::TimestampError(ValidateTimestampError::TimestampTooFarInTheFuture)))]
    #[case("rejected for timestamp is too old", 200, &VALID_PUBKEY, "Hello, Bob", 14, (Utc::now() - Duration::days(8)).timestamp() as u64, None, Err(SendMessageUseCaseError::TimestampError(ValidateTimestampError::TimestampTooOld)))]
    #[case("rejected for invalid public key's length", 200, "npub100", "Hello", 14, Utc::now().timestamp() as u64, None, Err(SendMessageUseCaseError::PublicKeyError(ValidatePublicKeyError::InvalidPublicKeyLength)))]
    #[case("rejected for public key isn't hex-encoded", 200, "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz", "Hello", 14, Utc::now().timestamp() as u64, None, Err(SendMessageUseCaseError::PublicKeyError(ValidatePublicKeyError::PublicKeyNotHexEncoded)))]
    #[case("rejected for ID isn't equal to our generated ID", 200, &VALID_PUBKEY, "Hello, Bob", 14, Utc::now().timestamp() as u64, Some("invalid_event_id".to_string()), Err(SendMessageUseCaseError::EventIDError(ValidateEventIDError::EventIDMismatch)))]
    fn send_message(
        #[case] _label: &str,
        #[case] max_length: usize,
        #[case] public_key: &str,
        #[case] content: &str,
        #[case] kind: u32,
        #[case] created_at: u64,
        #[case] mocked_id: Option<String>,
        #[case] expected: Result<(), SendMessageUseCaseError>) {
        let mut id = generate_event_id(public_key, created_at.clone(), kind, &Vec::new(), content);
        if let Some(mocked_id) = mocked_id {
            id = mocked_id;
        }
        let use_case = NostrSendMessageUseCase { max_length };
        let message = Message::new(id, public_key.to_string(), content.to_string(), created_at, kind, Vec::new());
        let result = use_case.execute(message);

        assert_eq!(result, expected);
    }
}
