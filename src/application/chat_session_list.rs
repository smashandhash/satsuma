use crate::{
    domain::chat_session::ChatSession,
    infrastructure::chat_session_repository::ChatSessionRepository
};

pub trait ChatSessionListUseCase {
    fn execute(&self, chat_session_id: String) -> Vec<ChatSession>;
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
    fn execute(&self, chat_session_id: String) -> Vec<ChatSession> {
        self.repository.load(chat_session_id)
    }
}
