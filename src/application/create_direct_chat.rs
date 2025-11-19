use crate::{
    domain::chat_container::{
        ChatContainer,
        ChatContainerContext
    },
    infrastructure::chat_container_repository::{
        ChatContainerRepository,
        ChatContainerRepositoryError
    }
};
use md5::compute;

pub trait CreateDirectChatUseCase {
    fn execute(&self, sender_public_key: &str, recipient_public_key: &str) -> Result<ChatContainer, CreateDirectChatUseCaseError>;
}

pub struct CreateDirectChatUseCaseImplementation<R: ChatContainerRepository> {
    repository: R,
}

impl<R: ChatContainerRepository> CreateDirectChatUseCaseImplementation<R> {
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

impl<R: ChatContainerRepository> CreateDirectChatUseCase for CreateDirectChatUseCaseImplementation<R> {
    fn execute(&self, sender_public_key: &str, recipient_public_key: &str) -> Result<ChatContainer, CreateDirectChatUseCaseError> {
        if sender_public_key.is_empty() || recipient_public_key.is_empty() {
            return Err(CreateDirectChatUseCaseError::InvalidPublicKey);
        }
        let chat_container = ChatContainer::new(
            self.generate_chat_session_id(sender_public_key, recipient_public_key), 
            ChatContainerContext::Direct { other_public_key: recipient_public_key.to_string() },
            vec![sender_public_key.to_string(), recipient_public_key.to_string()],
            Vec::new()
        );

        self.repository
            .save(chat_container.clone())
            .map_err(CreateDirectChatUseCaseError::RepositoryError)?;

        Ok(chat_container)
    }
}

#[derive(Debug, PartialEq)]
pub enum CreateDirectChatUseCaseError {
    InvalidPublicKey,
    RepositoryError(ChatContainerRepositoryError)
}
