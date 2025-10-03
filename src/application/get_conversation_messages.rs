use crate::{
    domain::message::Message,
    infrastructure::message_repository::MessageRepository
};

pub struct GetConversationMessagesUseCase<'a> {
    repository: &'a dyn MessageRepository,
}

impl<'a> GetConversationMessagesUseCase<'a> {
    pub fn new(repository: &'a dyn MessageRepository) -> Self {
        Self { repository }
    }

    pub fn execute(&self, sender_id: u64, recipient_id: u64) -> Vec<Message> {
        self.repository.find_conversation(sender_id, recipient_id)
    }
}
