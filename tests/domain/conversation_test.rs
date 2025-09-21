use satsuma::domain::conversation::Conversation;
use satsuma::domain::message::Message;
use satsuma::domain::user::User;

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

    #[test]
    fn conversation_has_correct_participants() {
        let conversation = Conversation::new(1, vec![101, 202]);
        
        assert!(conversation.participant_ids.contains(&101));
        assert!(conversation.participant_ids.contains(&202));
    }

    #[test]
    fn reject_message_from_non_participant_conversation() {
        let mut conversation = Conversation::new(1, vec![101, 202]);
        let outsider_message = Message::new(1, 303, 101, "Hi, can I join?");

        assert!(!conversation.add_message(outsider_message));
        assert!(conversation.messages.is_empty());
    }

    #[test]
    fn add_participant_on_conversation() {
        let mut conversation = Conversation::new(1, vec![101, 202]);
        let user = User::new(1, "Alice");

        conversation.add_participant(user.id);

        assert_eq!(conversation.participant_ids, vec![101, 202, 1]);
    }
}
