#[cfg(test)]
mod tests {
    use satsuma::{
        application::send_message::SendMessageUseCase,
        domain::user::User
    };
    use chrono::Utc;

    fn make_sut(max_length: usize) -> (SendMessageUseCase, User) {
        let use_case = SendMessageUseCase::new(max_length);
        let sender = User::new("npub1", "Alice");
        (use_case, sender)
    }

    #[test]
    fn send_message_to_another_user() {
        let (use_case, sender) = make_sut(200);
        let result = use_case.execute(&sender, "Hello, Bob!");

        assert!(result.is_ok());
        let message = result.unwrap();
        assert_eq!(message.content, "Hello, Bob!");
    }

    #[test]
    fn send_message_rejected_for_empty_message() {
        let (use_case, sender) = make_sut(200);
        let result = use_case.execute(&sender,"");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Sender cannot send empty message");
    }

    #[test]
    fn send_message_to_self() {
        let (use_case, sender) = make_sut(200);
        let result = use_case.execute(&sender, "Note to myself");

        assert!(result.is_ok());
        let message = result.unwrap();
        assert_eq!(message.content, "Note to myself");
    }

    #[test]
    fn send_message_rejected_due_the_content_is_only_spaces() {
        let (use_case, sender) = make_sut(200);
        let result = use_case.execute(&sender, " ");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Sender cannot send empty message");
    }

    #[test]
    fn send_message_rejected_due_too_long() {
        let (use_case, sender) = make_sut(8);
        let result = use_case.execute(&sender, "Hello, Bob");

        assert_eq!(result.unwrap_err(), "Message too long".to_string());
    }

    #[test]
    fn send_message_with_recent_timestamp() {
        let (use_case, sender) = make_sut(200);
        let time_now = Utc::now().timestamp() as u64;

        let result = use_case.execute(&sender, "Hello");

        assert!(result.is_ok());
        let message = result.unwrap();
        assert!((time_now as i64 - message.created_at as i64).abs() <= 1);
    }
}
