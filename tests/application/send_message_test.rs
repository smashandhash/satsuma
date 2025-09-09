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

        let message = use_case.execute(&sender, &recipient, "Hello, Bob!");

        assert_eq!(message.sender_id, sender.id);
        assert_eq!(message.recipient_id, recipient.id);
        assert_eq!(message.content, "Hello, Bob!");
    }
}
