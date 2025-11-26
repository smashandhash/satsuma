#[cfg(test)]
mod tests {
    use satsuma::{
        application::add_participants_to_chat_container::{
            AddParticipantsToChatContainerUseCase,
            AddParticipantsToChatContainerUseCaseImplementation,
            AddParticipantsToChatContainerUseCaseError,
        },
        domain::chat_container::{
            ChatContainer,
            ChatContainerContext,
            ChatContainerGroupType,
            ChatContainerError
        },
        infrastructure::chat_container_repository::ChatContainerRepositoryError
    };
    use crate::helper::chat_container_repository_stub::ChatContainerRepositoryStub;

    #[test]
    fn add_participant_to_conversation() {
        let repository = ChatContainerRepositoryStub {
            simulated_error: None,
            mocked_chat_container: Some(ChatContainer::new(
                "id".to_string(), 
                ChatContainerContext::Group { 
                    group_type: ChatContainerGroupType::Private,
                    creator_public_key: "creator_public_key".to_string(),
                    admins_public_key: vec!["creator_public_key".to_string()]
                },
                Vec::new(),
                Vec::new()
                )),
        };
        let sut = AddParticipantsToChatContainerUseCaseImplementation::new(repository);

        let result = sut.execute(
            "chat_container_id".to_string(), 
            "creator_public_key".to_string(), 
            Vec::new()
        );

        assert!(result.is_ok());
    }

    #[test]
    fn chat_container_error_cannot_add_participants() {
        let repository = ChatContainerRepositoryStub {
            simulated_error: None,
            mocked_chat_container: Some(ChatContainer::new(
                "id".to_string(), 
                ChatContainerContext::Direct { 
                    other_public_key: "creator_public_key".to_string(),
                },
                Vec::new(),
                Vec::new()
                )),
        };
        let sut = AddParticipantsToChatContainerUseCaseImplementation::new(repository);

        let result = sut.execute(
            "chat_container_id".to_string(), 
            "creator_public_key".to_string(), 
            Vec::new()
        );

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AddParticipantsToChatContainerUseCaseError::ContainerError(ChatContainerError::DirectChatCannotAddParticipants));
    }

    #[test]
    fn repository_error_cannot_add_participants() {
        let repository = ChatContainerRepositoryStub {
            simulated_error: Some(ChatContainerRepositoryError::ContainerNotFound),
            mocked_chat_container: None,
        };
        let sut = AddParticipantsToChatContainerUseCaseImplementation::new(repository);

        let result = sut.execute(
            "chat_container_id".to_string(), 
            "creator_public_key".to_string(), 
            Vec::new()
        );

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AddParticipantsToChatContainerUseCaseError::RepositoryError(ChatContainerRepositoryError::ContainerNotFound));
    }
}
