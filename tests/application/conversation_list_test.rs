#[cfg(test)]
mod tests {
    use satsuma::application::conversation_list::ConversationListUseCase;
    use satsuma::domain::conversation::Conversation;
    use satsuma::domain::user::User;
    use satsuma::infrastructure::conversation_repository::ConversationRepository;

    #[test]
    fn show_conversation_list() {
        let conversation = vec![create_conversation()];
        let mut repository = ConversationRepositoryStub::new(conversation.clone());
        let user = User::new(1, "Alice");
        let mut use_case = ConversationListUseCase::new(&mut repository);

        let conversation_list = use_case.execute(user.id);

        assert_eq!(conversation_list, conversation);
    }

    fn create_conversation() -> Conversation {
        Conversation::new(1, 1, vec![1, 2, 3])
    }

    pub struct ConversationRepositoryStub {
        conversation_list: Vec<Conversation>,
    }

    impl ConversationRepositoryStub {
        pub fn new(conversation_list: Vec<Conversation>) -> Self {
            Self { conversation_list }
        }
    }

    impl ConversationRepository for ConversationRepositoryStub {
        fn save(&mut self, conversation: Conversation) {}

        fn load(&mut self, user_id: u64) -> Vec<Conversation> {
            self.conversation_list.iter().cloned().collect()
        }
    }
}
