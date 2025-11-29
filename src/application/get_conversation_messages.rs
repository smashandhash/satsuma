use crate::{
    domain::{
        message::Message,
        chat_session::ChatSessionContext,
    },
    infrastructure::chat_container_repository::{
        ChatContainerRepository,
        ChatContainerRepositoryError
    },
};

pub trait LoadChatContainerMessagesUseCase {
    fn execute(&self, chat_container_id: String) -> Result<Vec<Message>, LoadChatContainerMessagesUseCaseError>;
}

pub struct LoadChatContainerMessagesUseCaseImplementation<R: ChatContainerRepository> {
    pub repository: R,
}

impl<R: ChatContainerRepository> LoadChatContainerMessagesUseCase for LoadChatContainerMessagesUseCaseImplementation<R> {
    fn execute(&self, chat_container_id: String) -> Result<Vec<Message>, LoadChatContainerMessagesUseCaseError> {
        let chat_container = self.repository.load(chat_container_id).map_err(|e| LoadChatContainerMessagesUseCaseError::RepositoryError(e))?;

        Ok(chat_container.sessions.into_iter().map(|session| { session.context == ChatSessionContext::Root }))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoadChatContainerMessagesUseCaseError {
    RepositoryError(ChatContainerRepositoryError),
}
