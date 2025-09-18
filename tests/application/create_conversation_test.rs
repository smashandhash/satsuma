#[cfg(test)]
mod tests {
    use satsuma::application::create_conversation::CreateConversationUseCase;
    use satsuma::domain::conversation::Conversation;
    use satsuma::infrastructure::conversation_repository::ConversationRepository;

    #[test]
    fn create_conversation_between_two_users() {
        let mut repository = ConversationRepositoryStub::new();
        let mut use_case = CreateConversationUseCase::new(&mut repository);

        let conversation = use_case.execute(1, 2);
        assert_eq!(conversation.participant_ids, vec![1, 2]);
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
    }
}
