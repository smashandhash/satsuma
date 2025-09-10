use satsuma::application::send_message::SendMessageUseCase;
use satsuma::domain::user::User;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send_message_to_another_user() {
        let sender = User::new(1, "Alice");
        let recipient = User::new(2, "Bob");
        let use_case = SendMessageUseCase::new();

        let result = use_case.execute(&sender, &recipient, "Hello, Bob!");

        assert!(result.is_ok());
        let message = result.unwrap();
        assert_eq!(message.recipient_id, recipient.id);
        assert_eq!(message.content, "Hello, Bob!");
    }

    #[test]
    fn send_message_rejected_for_empty_message() {
        let sender = User::new(1, "Alice");
        let recipient = User::new(2, "Bob");
        let use_case = SendMessageUseCase::new();

        let result = use_case.execute(&sender, &recipient, "");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Sender cannot send empty message");
    }

    #[test]
    fn send_message_to_self() {
        let sender = User::new(1, "Alice");
        let use_case = SendMessageUseCase::new();

        let result = use_case.execute(&sender, &sender, "Note to myself");

        assert!(result.is_ok());
        let message = result.unwrap();
        assert_eq!(message.recipient_id, sender.id);
        assert_eq!(message.content, "Note to myself");
    }
}
