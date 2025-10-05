#[cfg(test)]
mod tests {
    use satsuma::{
        application::create_conversation::CreateConversationUseCase,
        application::create_conversation::CreateConversationUseCaseError,
        domain::conversation::Conversation,
        infrastructure::conversation_repository::ConversationRepository
    };
    use rstest::rstest;

    #[rstest]
    #[case("conversation between two users", "npub1234", "npub2134")]
    #[case("conversation to self", "npub1234", "npub1234")]
    fn create_conversation_between_two_users(
        #[case] _label: &str,
        #[case] sender_public_key: String,
        #[case] recipient_public_key: String
        ) {
        let mut repository = ConversationRepositoryStub::new();
        let mut use_case = CreateConversationUseCase::new(&mut repository);

        let conversation = use_case.execute(&sender_public_key, &recipient_public_key).unwrap();
        assert_eq!(conversation.participant_public_keys, vec![sender_public_key, recipient_public_key]);
    }

    #[test]
    fn create_conversation_failed_due_user_ids_are_zero() {
        let mut repository = ConversationRepositoryStub::new();
        let mut use_case = CreateConversationUseCase::new(&mut repository);
        let result = use_case.execute("npub1234", "");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CreateConversationUseCaseError::InvalidPublicKey);
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

        fn load(&mut self, _user_public_key: String) -> Vec<Conversation> {
            Vec::new()
        }
    }
}
