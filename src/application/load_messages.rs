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

pub trait LoadMessagesUseCase {
    fn execute(&self, chat_container_id: String) -> Result<Vec<Message>, LoadChatContainerMessagesUseCaseError>;
}

pub struct LoadMessagesUseCaseImplementation<R: ChatContainerRepository> {
    pub repository: R,
}

impl<R: ChatContainerRepository> LoadMessagesUseCase for LoadMessagesUseCaseImplementation<R> {
    fn execute(&self, chat_container_id: String) -> Result<Vec<Message>, LoadChatContainerMessagesUseCaseError> {
        let chat_container = self.repository.load(chat_container_id).map_err(|e| LoadChatContainerMessagesUseCaseError::RepositoryError(e))?;

        let messages = &chat_container.sessions
            .iter()
            .find(|session| session.context == ChatSessionContext::Root )
            .unwrap()
            .messages;

        Ok(messages.clone())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoadChatContainerMessagesUseCaseError {
    RepositoryError(ChatContainerRepositoryError),
}
