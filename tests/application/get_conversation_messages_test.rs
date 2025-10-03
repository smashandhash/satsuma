#[cfg(test)]
mod tests {
    use satsuma::{
        application::get_conversation_messages::GetConversationMessagesUseCase,
        domain::message::Message,
        infrastructure::message_repository::MessageRepository
    };
    use rstest::rstest;

    #[rstest]
    #[case("conversation between two users", vec![Message::new(1, 1, 2, "Hello, Bob"), Message::new(2, 2, 1, "Hello, Alice")], 1, 2, 2)]
    #[case("empty conversation", Vec::new(), 1, 2, 0)]
    #[case("self conversation", vec![Message::new(1, 1, 1, "Note to myself")], 1, 1, 1)]
    #[case("conversation of different ID", vec![Message::new(1, 2, 3, "Hello!"), Message::new(2, 3, 2, "Hi!")], 1, 2, 0)]
    fn get_conversation_messages(
        #[case] _label: &str,
        #[case] messages: Vec<Message>,
        #[case] sender_id: u64,
        #[case] recipient_id: u64,
        #[case] conversation_length: usize
        ) {
        let repository = MessageRepositoryStub::new(messages.clone());
        let use_case = GetConversationMessagesUseCase::new(&repository);

        let conversation = use_case.execute(sender_id, recipient_id);

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
