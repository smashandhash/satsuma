use satsuma::domain::conversation::Conversation;
use satsuma::domain::message::Message;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversation_init_should_do_nothing() {
        let conversation = Conversation::new(1, vec![101, 202]);
    }

    #[test]
    fn can_add_message_to_conversation() {
        let mut conversation = Conversation::new(1, vec![101, 202]);
        let message = Message::new(1, 101, 202, "Hello!");

        conversation.add_message(message.clone());

        assert_eq!(conversation.messages.len(), 1);
        assert_eq!(conversation.messages[0].content, "Hello!");
    }

    #[test]
    fn message_should_in_order() {
        let mut conversation = Conversation::new(1, vec![101, 202]);

        let firstMessage = Message::new(1, 101, 202, "First");
        let secondMessage = Message::new(2, 202, 101, "Second");

        conversation.add_message(firstMessage);
        conversation.add_message(secondMessage);

        assert_eq!(conversation.messages[0].content, "First");
        assert_eq!(conversation.messages[1].content, "Second");
    }
}
