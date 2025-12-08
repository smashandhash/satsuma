#[cfg(test)]
mod tests {
    use satsuma::{
        application::remove_participants_from_chat_container::{
            RemoveParticipantsFromChatContainerUseCase,
            RemoveParticipantsFromChatContainerUseCaseImplementation,
            RemoveParticipantsFromChatContainerUseCaseError,
        },
        domain::chat_container::{
            ChatContainer,
            ChatContainerContext,
            ChatContainerGroupType,
            ChatContainerError,
        },
    };
    use crate::helper::chat_container_repository_stub::ChatContainerRepositoryStub;
    use std::sync::Arc;

    #[test]
    fn remove_existing_participant() {
        let creator_public_key = "creator_public_key".to_string();
        let target_public_key = "target_public_key".to_string();
        let chat_container_id = "id".to_string();
        let chat_container = ChatContainer::new(
            chat_container_id.clone(),
            ChatContainerContext::Group {
                group_type: ChatContainerGroupType::Private,
                creator_public_key: creator_public_key.clone(),
                admins_public_key: vec![creator_public_key.clone()],
            },
            vec![creator_public_key.clone(), target_public_key.clone(), "other_public_key".to_string()]
            );
        let repository = Arc::new(ChatContainerRepositoryStub {
            simulated_error: None,
            mocked_chat_container: Some(chat_container.clone()),
        });
        let sut = RemoveParticipantsFromChatContainerUseCaseImplementation::new(repository);

        let result = sut.execute(chat_container_id.clone(), creator_public_key.clone(), vec![target_public_key.clone()]);

        assert!(result.is_ok());
    }

    #[test]
    fn non_creator_unable_to_remove_existing_participant() {
        let creator_public_key = "creator_public_key".to_string();
        let target_public_key = "target_public_key".to_string();
        let chat_container_id = "id".to_string();
        let chat_container = ChatContainer::new(
            chat_container_id.clone(),
            ChatContainerContext::Group {
                group_type: ChatContainerGroupType::Private,
                creator_public_key: creator_public_key.clone(),
                admins_public_key: vec![creator_public_key.clone()],
            },
            vec![creator_public_key.clone(), target_public_key.clone(), "other_public_key".to_string()]
            );
        let repository = Arc::new(ChatContainerRepositoryStub {
            simulated_error: None,
            mocked_chat_container: Some(chat_container.clone()),
        });
        let sut = RemoveParticipantsFromChatContainerUseCaseImplementation::new(repository);

        let result = sut.execute(chat_container_id.clone(), "other_public_key".to_string(), vec![target_public_key.clone()]);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), RemoveParticipantsFromChatContainerUseCaseError::ContainerError(ChatContainerError::PermissionDenied));
    }
}
