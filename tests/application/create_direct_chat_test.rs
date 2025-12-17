#[cfg(test)]
mod tests {
    use satsuma::{
        application::create_direct_chat::{
            CreateDirectChatUseCase,
            CreateDirectChatUseCaseImplementation,
            CreateDirectChatUseCaseError,
        },
        domain::chat_container::{
            ChatContainer,
            ChatContainerContext,
        },
        infrastructure::chat_container_repository::ChatContainerRepositoryError
    };
    use crate::helper::chat_container_repository_stub::ChatContainerRepositoryStub;
    use std::sync::Arc;

    #[test]
    fn successfully_create_direct_chat() {
        let sender_public_key = "sender_public_key".to_string();
        let recipient_public_key = "recipient_public_key".to_string();
        let chat_container = ChatContainer::new(
            "id".to_string(), 
            ChatContainerContext::Direct { other_public_key: recipient_public_key.clone() },
            vec![sender_public_key.clone(), recipient_public_key.clone()]);
        let repository = Arc::new(ChatContainerRepositoryStub::new(
            None,
            Some(chat_container.clone())
        ));
        let sut = CreateDirectChatUseCaseImplementation::new(repository);

        let result = sut.execute(&sender_public_key, &recipient_public_key);

        assert!(result.is_ok());

        let saved_chat_container = result.unwrap();
        assert_eq!(saved_chat_container.context, chat_container.clone().context);
        assert_eq!(saved_chat_container.participant_public_keys, chat_container.clone().participant_public_keys);
    }

    #[test]
    fn invalid_public_keys_failed_to_create_direct_chat() {
        let chat_container = ChatContainer::new(
            "id".to_string(),
            ChatContainerContext::Direct { other_public_key: "other_public_key".to_string() },
            Vec::new());
        let repository = Arc::new(ChatContainerRepositoryStub::new(
            None,
            Some(chat_container.clone())
        ));
        let sut = CreateDirectChatUseCaseImplementation::new(repository);

        let result = sut.execute("", "");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CreateDirectChatUseCaseError::InvalidPublicKey);
    }

    #[test]
    fn repository_error_failed_to_create_direct_chat() {
        let sender_public_key = "sender_public_key".to_string();
        let recipient_public_key = "recipient_public_key".to_string();
        let simulated_error = ChatContainerRepositoryError::SaveFailed;
        let repository = Arc::new(ChatContainerRepositoryStub::new(
            Some(simulated_error.clone()),
            None
        ));
        let sut = CreateDirectChatUseCaseImplementation::new(repository);

        let result = sut.execute(&sender_public_key, &recipient_public_key);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CreateDirectChatUseCaseError::RepositoryError(simulated_error));
    }
}
