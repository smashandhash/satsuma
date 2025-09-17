use crate::domain::conversation::Conversation;
use crate::infrastructure::conversation_repository::ConversationRepository;

pub struct CreateConversationUseCase<'a> {
    repository: &'a mut dyn ConversationRepository,
}

impl<'a> CreateConversationUseCase<'a> {
    pub fn new(repository: &'a mut dyn ConversationRepository) -> Self {
        Self { repository }
    }

    pub fn execute(&mut self, sender_id: u64, recipient_id: u64) -> Conversation {
        let conversation = Conversation::new(1, vec![sender_id, recipient_id]);
        self.repository.save(conversation.clone());
        conversation
    }
}
