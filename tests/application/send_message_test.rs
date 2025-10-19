#[cfg(test)]
mod tests {
    use satsuma::{
        application::send_message::SendMessageUseCase,
        application::send_message::SendMessageUseCaseError,
        application::send_message::NostrSendMessageUseCase,
        domain::user::User,
        domain::event_kind::EventKind,
        domain::services::generate_event_id::generate_event_id
    };
    use chrono::Utc;
    use rstest::rstest;

    fn make_sut(max_length: usize) -> (NostrSendMessageUseCase, User) {
        let use_case = NostrSendMessageUseCase { max_length: max_length };
        let sender = User::new("npub1", "Alice");
        (use_case, sender)
    }

    #[rstest]
    #[case("send message to another user", 200, "npub100", "Hello, Bob!", Ok(()))]
    #[case("rejected for empty message", 200, "npub100", "", Err(SendMessageUseCaseError::EmptyMessage))]
    #[case("rejected for message has only spaces", 200, "npub100", "   ", Err(SendMessageUseCaseError::EmptyMessage))]
    #[case("rejected for message is too long", 8, "npub100", "Hello, Bob", Err(SendMessageUseCaseError::MessageTooLong))]
    fn send_message(
        #[case] _label: &str,
        #[case] max_length: usize,
        #[case] public_key: &str,
        #[case] content: &str,
        #[case] expected: Result<(), SendMessageUseCaseError>) {
        let created_at = Utc::now().timestamp() as u64;
        let kind = EventKind::PrivateOrGroupMessage as u32;
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
