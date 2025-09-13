use crate::domain::message::Message;
use crate::infrastructure::message_repository::MessageRepository;

pub struct GetConversationMessagesUseCase<'a> {
    repository: &'a dyn MessageRepository,
}

impl<'a> GetConversationMessagesUseCase<'a> {
    pub fn new(repository: &'a dyn MessageRepository) -> Self {
        Self { repository }
    }

    pub fn execute(&self, sender_id: &str, recipient_id: &str) -> Vec<Message> {
        self.repository.find_conversation(sender_id, recipient_id)
    }
}
