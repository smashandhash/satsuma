/*
#[cfg(test)]
mod tests {
    use satsuma::{
        application::conversation_list::ConversationListUseCase,
        domain::conversation::Conversation,
        domain::user::User,
        infrastructure::conversation_repository::ConversationRepository
    };

    #[test]
    fn show_conversation_list() {
        let conversation = vec![create_conversation()];
        let mut repository = ConversationRepositoryStub::new(conversation.clone());
        let user = User::new("npub1234", "Alice");
        let mut use_case = ConversationListUseCase::new(&mut repository);

        let conversation_list = use_case.execute(user.public_key);

        assert_eq!(conversation_list, conversation);
    }

    #[test]
    fn do_not_show_conversation_list_with_different_id() {
        let conversation = vec![create_conversation()];
        let mut repository = ConversationRepositoryStub::new(conversation.clone());
        let user = User::new("npub4123", "Denise");
        let mut use_case = ConversationListUseCase::new(&mut repository);

        let conversation_list = use_case.execute(user.public_key);

        assert_ne!(conversation_list, conversation);
    }

    fn create_conversation() -> Conversation {
        Conversation::new(1, "npub1234", vec!["npub1234", "npub2134", "npub3124"])
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
        fn save(&mut self, _: Conversation) {}

        fn load(&mut self, _: String) -> Vec<Conversation> {
            self.conversation_list.iter().cloned().collect()
        }
    }
}
*/
