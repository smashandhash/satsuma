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
}
