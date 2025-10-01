use crate::domain::conversation::Conversation;
use crate::infrastructure::conversation_repository::ConversationRepository;

pub struct ConversationListUseCase<'a> {
    repository: &'a mut dyn ConversationRepository,
}

impl<'a> ConversationListUseCase<'a> {
    pub fn new(repository: &'a mut dyn ConversationRepository) -> Self {
        Self { repository }
    }

    pub fn execute(&mut self, user_id: u64) -> Vec<Conversation> {
        self.repository.load(user_id).clone()
    }
}
