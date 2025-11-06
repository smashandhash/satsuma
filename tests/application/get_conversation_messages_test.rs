#[cfg(test)]
mod tests {
    use satsuma::{
        application::get_conversation_messages::GetConversationMessagesUseCase,
        domain::message::Message,
        infrastructure::message_repository::{
            MessageRepository,
            MessageRepositoryError
        }
    };
    use chrono::Utc;
    use rstest::rstest;
    use async_trait::async_trait;

    #[rstest]
    #[case("conversation between two users", vec![Message::new("id1".to_string(), "npub1234".to_string(), "Hello, Bob".to_string(), Utc::now().timestamp() as u64, 14, Vec::new(), "".to_string()), Message::new("id2".to_string(), "npub2134".to_string(), "Hello, Alice".to_string(), Utc::now().timestamp() as u64, 14, Vec::new(), "".to_string())], "npub1234", "npub2134", 2)]
    #[case("empty conversation", Vec::new(), "npub1234", "npub2134", 0)]
    #[case("self conversation", vec![Message::new("id1".to_string(), "npub1234".to_string(), "Note to myself".to_string(), Utc::now().timestamp() as u64, 14, Vec::new(), "".to_string())], "npub1234", "npub1234", 1)]
    #[case("conversation of different ID", vec![Message::new("id1".to_string(), "npub2134".to_string(), "Hello!".to_string(), Utc::now().timestamp() as u64, 14, Vec::new(), "".to_string()), Message::new("id2".to_string(), "npub3124".to_string(), "Hi!".to_string(), Utc::now().timestamp() as u64, 14, Vec::new(), "".to_string())], "npub1234", "npub2134", 1)]
    fn get_conversation_messages(
        #[case] _label: &str,
        #[case] messages: Vec<Message>,
        #[case] sender_public_key: &str,
        #[case] recipient_public_key: &str,
        #[case] conversation_length: usize
        ) {
        let repository = MessageRepositoryStub::new(messages.clone());
        let use_case = GetConversationMessagesUseCase { repository: repository };

        let conversation = use_case.execute(sender_public_key.to_string(), recipient_public_key.to_string());

        assert_eq!(conversation.len(), conversation_length);
    }

    pub struct MessageRepositoryStub {
        messages: Vec<Message>,
    }

    impl MessageRepositoryStub {
        pub fn new(messages: Vec<Message>) -> Self {
            Self { messages }
        }
    }

    #[async_trait]
    impl MessageRepository for MessageRepositoryStub {
        async fn send(&self, _message: &Message) -> Result<(), MessageRepositoryError> {
            Ok(())
        }

        fn find_conversation(&self, sender_public_key: String, recipient_public_key: String) -> Vec<Message> {
            self.messages
                .iter()
                .filter( |m| {
                    (m.public_key == sender_public_key) || (m.public_key == recipient_public_key)
                })
                .cloned()
                    .collect()
        }
    }
}
