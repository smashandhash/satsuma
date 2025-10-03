#[cfg(test)]
mod tests {
    use satsuma::{
        application::send_message::SendMessageUseCase,
        domain::user::User
    };
    use chrono::Utc;

    fn make_sut(is_send_to_self: bool, max_length: usize) -> (SendMessageUseCase, User, User) {
        let use_case = SendMessageUseCase::new(max_length);
        let sender = User::new(1, "Alice");
        let recipient = if is_send_to_self {
            sender.clone()
        } else {
            User::new(2, "Bob")
        };
        (use_case, sender, recipient)
    }

    #[test]
    fn send_message_to_another_user() {
        let (use_case, sender, recipient) = make_sut(false, 200);
        let result = use_case.execute(&sender, &recipient, "Hello, Bob!");

        assert!(result.is_ok());
        let message = result.unwrap();
        assert_eq!(message.recipient_id, recipient.id);
        assert_eq!(message.content, "Hello, Bob!");
    }

    #[test]
    fn send_message_rejected_for_empty_message() {
        let (use_case, sender, recipient) = make_sut(false, 200);
        let result = use_case.execute(&sender, &recipient, "");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Sender cannot send empty message");
    }

    #[test]
    fn send_message_to_self() {
        let (use_case, sender, recipient) = make_sut(true, 200);
        let result = use_case.execute(&sender, &recipient, "Note to myself");

        assert!(result.is_ok());
        let message = result.unwrap();
        assert_eq!(message.recipient_id, recipient.id);
        assert_eq!(message.content, "Note to myself");
    }

    #[test]
    fn send_message_rejected_due_the_content_is_only_spaces() {
        let (use_case, sender, recipient) = make_sut(false, 200);
        let result = use_case.execute(&sender, &recipient, " ");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Sender cannot send empty message");
    }

    #[test]
    fn send_message_rejected_due_too_long() {
        let (use_case, sender, recipient) = make_sut(false, 8);
        let result = use_case.execute(&sender, &recipient, "Hello, Bob");

        assert_eq!(result.unwrap_err(), "Message too long".to_string());
    }

    #[test]
    fn send_message_with_recent_timestamp() {
        let (use_case, sender, recipient) = make_sut(false, 200);
        let time_now = Utc::now();

        let result = use_case.execute(&sender, &recipient, "Hello");

        assert!(result.is_ok());
        let message = result.unwrap();
        assert!((time_now - message.timestamp).num_seconds().abs() < 5);
    }
}
