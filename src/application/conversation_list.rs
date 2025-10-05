use crate::{
    domain::conversation::Conversation,
    infrastructure::conversation_repository::ConversationRepository
};

pub struct ConversationListUseCase<'a> {
    repository: &'a mut dyn ConversationRepository,
}

impl<'a> ConversationListUseCase<'a> {
    pub fn new(repository: &'a mut dyn ConversationRepository) -> Self {
        Self { repository }
    }

    pub fn execute(&mut self, user_public_key: String) -> Vec<Conversation> {
        self.repository.load(user_public_key.clone()).iter()
            .filter( |conversation| conversation.participant_public_keys.contains(&user_public_key) )
            .cloned().collect()
    }
}
