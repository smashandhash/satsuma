use crate::{
    domain::chat_container::ChatContainerError,
    infrastructure::chat_container_repository::{
        ChatContainerRepository,
        ChatContainerRepositoryError
    }
};

pub trait AddParticipantsToChatContainer {
    fn execute(&self, chat_container_id: String, actor_public_key: String, new_participant_public_keys: Vec<String>) -> Result<(), AddParticipantsToChatContainerUseCaseError>;
}

pub struct AddParticipantsToChatContainerUseCaseImplementation<R: ChatContainerRepository> {
    repository: R
}

impl<R: ChatContainerRepository> AddParticipantsToChatContainer for AddParticipantsToChatContainerUseCaseImplementation <R> {
    fn execute(&self, chat_container_id: String, actor_public_key: String, new_participant_public_keys: Vec<String>) -> Result<(), AddParticipantsToChatContainerUseCaseError> {
        let mut chat_container = self.repository
            .load(chat_container_id)
            .map_err(|e| AddParticipantsToChatContainerUseCaseError::RepositoryError(e))
            .unwrap();

        chat_container
            .add_participants(&actor_public_key, new_participant_public_keys)
            .map_err(|e| AddParticipantsToChatContainerUseCaseError::ContainerError(e));

        self.repository.save(chat_container).map_err(|e| AddParticipantsToChatContainerUseCaseError::RepositoryError(e))
    }
}

#[derive(Debug, PartialEq)]
pub enum AddParticipantsToChatContainerUseCaseError {
    ContainerError(ChatContainerError),
    RepositoryError(ChatContainerRepositoryError)
}
