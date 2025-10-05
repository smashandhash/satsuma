use crate::{
    domain::conversation::Conversation,
    infrastructure::conversation_repository::ConversationRepository
};

pub struct CreateConversationUseCase<'a> {
    repository: &'a mut dyn ConversationRepository,
}

impl<'a> CreateConversationUseCase<'a> {
    pub fn new(repository: &'a mut dyn ConversationRepository) -> Self {
        Self { repository }
    }

    pub fn execute(&mut self, sender_public_key: &str, recipient_public_key: &str) -> Result<Conversation, CreateConversationUseCaseError> {
        if sender_public_key == "" || recipient_public_key == "" {
            return Err(CreateConversationUseCaseError::InvalidPublicKey);
        }
        let conversation = Conversation::new(1, sender_public_key, vec![sender_public_key, recipient_public_key]);
        self.repository.save(conversation.clone());
        Ok(conversation)
    }
}

#[derive(Debug, PartialEq)]
pub enum CreateConversationUseCaseError {
    InvalidPublicKey
}
