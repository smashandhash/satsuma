#[cfg(test)]
mod tests {
    use satsuma::application::get_conversation_messages::GetConversationMessageUseCase;
    use satsuma::domain::message::Message;
    use satsuma::infrastructure::message_repository::MessageRepository;

    #[test]
    fn get_conversation_messages_between_two_users() {
        let messages = vec![
            Message::new(1, 1, 2, "Hello, Bob"),
            Message::new(2, 2, 1, "Hello, Alice"),
        ];
        let repository = MessageRepositoryStub::new(messages.clone());
        let use_case = GetConversationMessagesUseCase::new(&repo);

        let conversation = use_case.execute("alice", "bob");

        assert_eq!(conversation.len(), 2);
        assert_eq!(conversation[0].content, "Hello, Bob");
        assert_eq!(conversation[1].content, "Hello, Alice");
    }

    pub struct MessageRepositoryStub {
        messages: Vec<Message>,
    }

    impl MessageRepositoryStub {
        pub fn new(messages: Vec<Message>) -> Self {
            Self { messages }
        }
    }

    impl MessageRepositoryStub for MessageRepository {
        fn find_conversation(&self, sender_id: &str, recipient_id: &str) -> Vec<Message> {
            self.messages.clone()
        }
    }
}
