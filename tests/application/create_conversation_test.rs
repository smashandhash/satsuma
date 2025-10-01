#[cfg(test)]
mod tests {
    use satsuma::application::create_conversation::CreateConversationUseCase;
    use satsuma::domain::conversation::Conversation;
    use satsuma::infrastructure::conversation_repository::ConversationRepository;
    use rstest::rstest;

    #[rstest]
    #[case("conversation between two users", 1, 2)]
    #[case("conversation to self", 1, 1)]
    fn create_conversation_between_two_users(
        #[case] _label: &str,
        #[case] sender_id: u64,
        #[case] recipient_id: u64
        ) {
        let mut repository = ConversationRepositoryStub::new();
        let mut use_case = CreateConversationUseCase::new(&mut repository);

        let conversation = use_case.execute(sender_id, recipient_id).unwrap();
        assert_eq!(conversation.participant_ids, vec![sender_id, recipient_id]);
    }

    #[test]
    fn create_conversation_failed_due_user_ids_are_zero() {
        let mut repository = ConversationRepositoryStub::new();
        let mut use_case = CreateConversationUseCase::new(&mut repository);
        let result = use_case.execute(1, 0);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Participants cannot be zero");
    }

    struct ConversationRepositoryStub {
        saved: Option<Conversation>,
    }

    impl ConversationRepositoryStub {
        fn new() -> Self {
            Self { saved: None }
        }
    }

    impl ConversationRepository for ConversationRepositoryStub {
        fn save(&mut self, conversation: Conversation) {
            self.saved = Some(conversation);
        }

        fn load(&mut self, user_id: u64) -> Vec<Conversation> {
            Vec::new()
        }
    }
}
