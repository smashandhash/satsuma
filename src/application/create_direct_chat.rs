use crate::{
    domain::chat_container::{
        ChatContainer,
        ChatContainerType
    },
    infrastructure::chat_session_repository::ChatSessionRepository
};
use md5::compute;

pub trait CreateDirectChatUseCase {
    fn execute(&self, sender_public_key: &str, recipient_public_key: &str) -> Result<ChatSession, CreateDirectChatUseCaseError>;
}

pub struct CreateDirectChatUseCaseImplementation<R: ChatSessionRepository> {
    repository: R,
}

impl<R: ChatSessionRepository> CreateDirectChatUseCaseImplementation<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    fn generate_chat_session_id(&self, sender_public_key: &str, recipient_public_key: &str) -> String {
        let mut keys = vec![sender_public_key.to_string(), recipient_public_key.to_string()];
        keys.sort();

        let joined_keys = format!("{}:{}", keys[0], keys[1]);
        format!("{:x}", compute(joined_keys))
    }
}

impl<R: ChatSessionRepository> CreateDirectChatUseCase for CreateDirectChatUseCaseImplementation<R> {
    fn execute(&self, sender_public_key: &str, recipient_public_key: &str) -> Result<ChatSession, CreateDirectChatUseCaseError> {
        if sender_public_key.is_empty() || recipient_public_key.is_empty() {
            return Err(CreateDirectChatUseCaseError::InvalidPublicKey);
        }
        let chat_container = ChatContainer::new(
            self.generate_chat_session_id(sender_public_key, recipient_public_key), 
            ChatContainerType::Direct,
            Vec::new()
        );
        self.repository.save(chat_session.clone());
        Ok(chat_session)
    }
}

#[derive(Debug, PartialEq)]
pub enum CreateDirectChatUseCaseError {
    InvalidPublicKey
}
