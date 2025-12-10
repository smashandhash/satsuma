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
    fn new(repository: Arc<R>) -> Self {
        Self {
            repository
        }
    }
}

impl<R: ChatContainerRepository> SearchChatUseCase for SearchChatUseCaseImplementation<R> {
    fn execute(&self, keyword: String) -> Result<Vec<ChatContainer>, SearchChatUseCaseError> {
        // TODO: Learn on how to implement search logic
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SearchChatUseCaseError {
    RepositoryError(ChatContainerRepositoryError),
}
