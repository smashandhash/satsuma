#[cfg(test)]
mod tests {
    use satsuma::{
        application::send_message::SendMessageUseCase,
        application::send_message::SendMessageUseCaseError,
        application::send_message::NostrSendMessageUseCase,
        domain::services::generate_event_id::generate_event_id,
        domain::services::validate_timestamp::ValidateTimestampError,
        domain::services::validate_public_key::ValidatePublicKeyError
    };
    use chrono::{Utc, Duration};
    use rstest::rstest;

    const VALID_PUBKEY: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    #[rstest]
    #[case("send message to another user", 200, &VALID_PUBKEY, "Hello, Bob!", 14, Utc::now().timestamp() as u64, Ok(()))]
    #[case("rejected for empty message", 200, &VALID_PUBKEY, "", 14, Utc::now().timestamp() as u64, Err(SendMessageUseCaseError::EmptyMessage))]
    #[case("rejected for message has only spaces", 200, &VALID_PUBKEY, "   ", 14, Utc::now().timestamp() as u64, Err(SendMessageUseCaseError::EmptyMessage))]
    #[case("rejected for message is too long", 8, &VALID_PUBKEY, "Hello, Bob", 14, Utc::now().timestamp() as u64, Err(SendMessageUseCaseError::MessageTooLong))]
    #[case("rejected for message's kind not found", 200, &VALID_PUBKEY, "Hello, Bob", 5000, Utc::now().timestamp() as u64, Err(SendMessageUseCaseError::KindNotFound("Invalid kind value: 5000".to_string())))]
    #[case("rejected for timestamp is invalid", 200, &VALID_PUBKEY, "Hello, Bob", 14, u64::MAX, Err(SendMessageUseCaseError::TimestampError(ValidateTimestampError::InvalidTimestamp)))]
    #[case("rejected for timestamp is far in the future", 200, &VALID_PUBKEY, "Hello, Bob", 14, (Utc::now() + Duration::seconds(60 * 10)).timestamp() as u64, Err(SendMessageUseCaseError::TimestampError(ValidateTimestampError::TimestampTooFarInTheFuture)))]
    #[case("rejected for timestamp is too old", 200, &VALID_PUBKEY, "Hello, Bob", 14, (Utc::now() - Duration::days(8)).timestamp() as u64, Err(SendMessageUseCaseError::TimestampError(ValidateTimestampError::TimestampTooOld)))]
    #[case("rejected for invalid public key's length", 200, "npub100", "Hello", 14, Utc::now().timestamp() as u64, Err(SendMessageUseCaseError::PublicKeyError(ValidatePublicKeyError::InvalidPublicKeyLength)))]
    fn send_message(
        #[case] _label: &str,
        #[case] max_length: usize,
        #[case] public_key: &str,
        #[case] content: &str,
        #[case] kind: u32,
        #[case] created_at: u64,
        #[case] expected: Result<(), SendMessageUseCaseError>) {
        let id = generate_event_id(public_key, created_at.clone(), kind, &Vec::new(), content);
        let use_case = NostrSendMessageUseCase { max_length };
        let result = use_case.execute(&id, public_key, content, &created_at, &kind, &Vec::new());

        assert_eq!(result, expected);
    }

    /*
    #[test]
    fn send_message_with_recent_timestamp() {
        let (use_case, sender) = make_sut(200);
        let time_now = Utc::now().timestamp() as u64;

        let result = use_case.execute(&sender, "Hello");

        assert!(result.is_ok());
        let message = result.unwrap();
        assert!((time_now as i64 - message.created_at as i64).abs() <= 1);
    }
    */
}
