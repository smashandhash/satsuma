use crate::{
    domain::message::Message,
    infrastructure::message_repository::MessageRepository
};

pub struct GetConversationMessagesUseCase<R: MessageRepository> {
    pub repository: R,
}

impl<R: MessageRepository> GetConversationMessagesUseCase<R> {
    pub fn execute(&self, sender_public_key: String, recipient_public_key: String) -> Vec<Message> {
        self.repository.find_conversation(sender_public_key, recipient_public_key)
    }
}
