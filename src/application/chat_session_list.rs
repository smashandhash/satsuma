use crate::{
    domain::chat_session::ChatSession,
    infrastructure::chat_session_repository::{
        ChatSessionRepository,
        ChatSessionRepositoryError
    },
};

pub trait ChatSessionListUseCase {
    fn execute(&self, container_id: String) -> Result<Vec<ChatSession>, ChatSessionListUseCaseError>;
}

pub struct ChatSessionListUseCaseImplementation <R: ChatSessionRepository> {
    repository: R,
}

impl<R: ChatSessionRepository> ChatSessionListUseCaseImplementation <R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: ChatSessionRepository> ChatSessionListUseCase for ChatSessionListUseCaseImplementation<R> {
    fn execute(&self, container_id: String) -> Result<Vec<ChatSession>, ChatSessionListUseCaseError> {
        self.repository.load_by_container_id(container_id).map_err(|e| ChatSessionListUseCaseError::RepositoryError(e))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChatSessionListUseCaseError {
    RepositoryError(ChatSessionRepositoryError)
}
