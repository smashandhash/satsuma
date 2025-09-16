#[cfg(test)]
mod tests {
    use satsuma::application::get_conversation_messages::GetConversationMessagesUseCase;
    use satsuma::domain::message::Message;
    use satsuma::infrastructure::message_repository::MessageRepository;

    #[test]
    fn get_conversation_messages_between_two_users() {
        let messages = vec![
            Message::new(1, 1, 2, "Hello, Bob"),
            Message::new(2, 2, 1, "Hello, Alice"),
        ];
        let repository = MessageRepositoryStub::new(messages.clone());
        let use_case = GetConversationMessagesUseCase::new(&repository);

        let conversation = use_case.execute(1, 2);

        assert_eq!(conversation.len(), 2);
        assert_eq!(conversation[0].content, "Hello, Bob");
        assert_eq!(conversation[1].content, "Hello, Alice");
    }

    #[test]
    fn get_empty_conversation_messages() {
        let messages = Vec::new();
        let repository = MessageRepositoryStub::new(messages.clone());
        let use_case = GetConversationMessagesUseCase::new(&repository);
        let conversation = use_case.execute(1, 2);
        assert_eq!(conversation.len(), 0);
    }

    #[test]
    fn get_self_conversation_messages() {
        let messages = vec![
            Message::new(1, 1, 1, "Note to myself"),
        ];
        let repository = MessageRepositoryStub::new(messages);
        let use_case = GetConversationMessagesUseCase::new(&repository);
        let conversation = use_case.execute(1, 1);

        assert_eq!(conversation.len(), 1);
        assert_eq!(conversation[0].content, "Note to myself");
    }

    #[test]
    fn get_conversation_messages_not_showing_due_to_different_ids() {
        let messages = vec![
            Message::new(1, 2, 3, "Hello"),
            Message::new(2, 3, 2, "Hi"),
        ];
        let repository = MessageRepositoryStub::new(messages.clone());
        let use_case = GetConversationMessagesUseCase::new(&repository);
        let conversation = use_case.execute(1, 2);

        assert_eq!(conversation.len(), 0);
    }

    pub struct MessageRepositoryStub {
        messages: Vec<Message>,
    }

    impl MessageRepositoryStub {
        pub fn new(messages: Vec<Message>) -> Self {
            Self { messages }
        }
    }

    impl MessageRepository for MessageRepositoryStub {
        fn find_conversation(&self, sender_id: u64, recipient_id: u64) -> Vec<Message> {
            self.messages
                .iter()
                .filter( |m| {
                    (m.sender_id == sender_id && m.recipient_id == recipient_id) || (m.sender_id == recipient_id && m.recipient_id == sender_id)
                })
                .cloned()
                    .collect()
        }
    }
}
