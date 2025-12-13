use crate::{
    domain::chat_container::ChatContainer,
    infrastructure::chat_container_repository::{
        ChatContainerRepository,
        ChatContainerRepositoryError,
    },
};
use std::sync::Arc;

pub trait SearchChatUseCase {
    fn execute(&self, keyword: String) -> Result<Vec<ChatContainer>, SearchChatUseCaseError>;
}

pub struct SearchChatUseCaseImplementation<R: ChatContainerRepository> {
    repository: Arc<R>,
}

impl<R: ChatContainerRepository> SearchChatUseCaseImplementation<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self {
            repository
        }
    }
}

impl<R: ChatContainerRepository> SearchChatUseCase for SearchChatUseCaseImplementation<R> {
    fn execute(&self, keyword: String) -> Result<Vec<ChatContainer>, SearchChatUseCaseError> {
        let containers = self.repository.search(keyword).map_err(|e| SearchChatUseCaseError::RepositoryError(e))?;

        Ok(containers)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SearchChatUseCaseError {
    RepositoryError(ChatContainerRepositoryError),
}
