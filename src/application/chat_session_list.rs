use crate::{
    domain::chat_session::ChatSession,
    infrastructure::chat_session_repository::ChatSessionRepository
};

pub trait ChatSessionListUseCase {
    fn execute(&self, user_public_key: &str) -> Vec<ChatSession>;
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
    fn execute(&self, user_public_key: &str) -> Vec<ChatSession> {
        self.repository.load(user_public_key.to_string())
            .into_iter()
            .filter( |chat_session| chat_session.participant_public_keys.contains(&user_public_key.to_string()) )
            .collect()
    }
}
