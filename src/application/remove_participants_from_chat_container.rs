use crate::{
    domain::chat_container::ChatContainerError,
    infrastructure::chat_container_repository::{
        ChatContainerRepository,
        ChatContainerRepositoryError
    },
};

pub trait RemoveParticipantsFromChatContainerUseCase {
    fn execute(&self, chat_container_id: String, actor_public_key: String, participant_public_keys: Vec<String>) -> Result<(), RemoveParticipantsFromChatContainerUseCaseError>;
}

pub struct RemoveParticipantsFromChatContainerUseCaseImplementation<R: ChatContainerRepository> {
    repository: R,
}

impl<R: ChatContainerRepository> RemoveParticipantsFromChatContainerUseCase for RemoveParticipantsFromChatContainerUseCaseImplementation <R> {
    fn execute(&self, chat_container_id: String, actor_public_key: String, participant_public_keys: Vec<String>) -> Result<(), RemoveParticipantsFromChatContainerUseCaseError> {
        let mut chat_container = self.repository
            .load(chat_container_id)
            .map_err(|e| RemoveParticipantsFromChatContainerUseCaseError::RepositoryError(e))
            .unwrap();

        chat_container.remove_participants(&actor_public_key, participant_public_keys)
            .map_err(|e| RemoveParticipantsFromChatContainerUseCaseError::ContainerError(e))?;

        self.repository.save(chat_container).map_err(|e| RemoveParticipantsFromChatContainerUseCaseError::RepositoryError(e))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RemoveParticipantsFromChatContainerUseCaseError {
    ContainerError(ChatContainerError),
    RepositoryError(ChatContainerRepositoryError)
}
