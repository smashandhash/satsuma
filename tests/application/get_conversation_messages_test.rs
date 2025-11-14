#[cfg(test)]
mod tests {
    use satsuma::{
        application::get_conversation_messages::GetConversationMessagesUseCase,
        domain::message::{
            Message,
            MessageKind
        }
    };
    use crate::helper::message_repository_stub::MessageRepositoryStub;
    use chrono::Utc;
    use rstest::rstest;

    #[rstest]
    #[case("conversation between two users", vec![Message::new("id1".to_string(), "npub1234".to_string(), "Hello, Bob".to_string(), Utc::now().timestamp() as u64, MessageKind::Direct("npub2134".to_string())), Message::new("id2".to_string(), "npub2134".to_string(), "Hello, Alice".to_string(), Utc::now().timestamp() as u64, MessageKind::Direct("npub1234".to_string()))], "npub1234", "npub2134", 2)]
    #[case("empty conversation", Vec::new(), "npub1234", "npub2134", 0)]
    #[case("self conversation", vec![Message::new("id1".to_string(), "npub1234".to_string(), "Note to myself".to_string(), Utc::now().timestamp() as u64, MessageKind::Direct("npub1234".to_string()))], "npub1234", "npub1234", 1)]
    #[case("conversation of different ID", vec![Message::new("id1".to_string(), "npub2134".to_string(), "Hello!".to_string(), Utc::now().timestamp() as u64, MessageKind::Direct("npub3124".to_string())), Message::new("id2".to_string(), "npub3124".to_string(), "Hi!".to_string(), Utc::now().timestamp() as u64, MessageKind::Direct("npub2134".to_string()))], "npub1234", "npub2134", 1)]
    fn get_conversation_messages(
        #[case] _label: &str,
        #[case] messages: Vec<Message>,
        #[case] sender_public_key: &str,
        #[case] recipient_public_key: &str,
        #[case] conversation_length: usize
        ) {
        let repository = MessageRepositoryStub::new(None, messages.clone());
        let use_case = GetConversationMessagesUseCase { repository };

        let conversation = use_case.execute(sender_public_key.to_string(), recipient_public_key.to_string());

        assert_eq!(conversation.len(), conversation_length);
    }
}
