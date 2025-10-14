#[cfg(test)]
mod tests {
    use satsuma::domain::{
        conversation::Conversation,
        message::Message,
        user::User
    };

    #[test]
    fn conversation_init_should_do_nothing() {
        let _conversation = Conversation::new(1, "npub101", vec!["npub101", "npub202"]);
    }

    #[test]
    fn can_add_message_to_conversation() {
        let mut conversation = Conversation::new(1, "npub101", vec!["npub101", "npub202"]);
        let message = Message::new(1, "npub101", "Hello!");

        conversation.add_message(message.clone());

        assert_eq!(conversation.messages.len(), 1);
        assert_eq!(conversation.messages[0].content, "Hello!");
    }

    #[test]
    fn message_should_in_order() {
        let mut conversation = Conversation::new(1, "npub101", vec!["npub101", "npub202"]);

        let first_message = Message::new(1, "npub101", "First");
        let second_message = Message::new(2, "npub202", "Second");

        conversation.add_message(first_message.clone());
        conversation.add_message(second_message.clone());

        assert_eq!(conversation.messages[0].content, first_message.content);
        assert_eq!(conversation.messages[1].content, second_message.content);
    }

    #[test]
    fn conversation_has_correct_participants() {
        let conversation = Conversation::new(1, "npub101", vec!["npub101", "npub202"]);
        
        assert!(conversation.participant_public_keys.iter().any(|public_key| public_key == "npub101"));
        assert!(conversation.participant_public_keys.iter().any(|public_key| public_key == "npub202"));
    }

    #[test]
    fn reject_message_from_non_participant_conversation() {
        let mut conversation = Conversation::new(1, "npub101", vec!["npub101", "npub202"]);
        let outsider_message = Message::new(1, "npub303", "Hi, can I join?");

        assert!(!conversation.add_message(outsider_message));
        assert!(conversation.messages.is_empty());
    }

    #[test]
    fn add_participant_on_conversation() {
        let mut conversation = Conversation::new(1, "npub101", vec!["npub101", "npub202"]);
        let user = User::new("npub1", "Alice");

        let result = conversation.add_participant(user.public_key);

        assert!(result.is_ok());
        assert_eq!(conversation.participant_public_keys, vec!["npub101", "npub202", "npub1"]);
    }

    #[test]
    fn remove_participant_on_conversation() {
        let mut conversation = Conversation::new(1, "npub1", vec!["npub1", "npub2", "npub3"]);
        let user_target = User::new("npub3", "Chad");

        conversation.remove_participant(user_target.public_key.to_string());

        assert!(!conversation.participant_public_keys.iter().any(|public_key| public_key.as_str() == user_target.public_key.as_str()));
    }
}
