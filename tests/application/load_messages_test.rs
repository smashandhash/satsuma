#[cfg(test)]
mod tests {
    use satsuma::{
        application::load_messages::{
            LoadMessagesUseCase,
            LoadMessagesUseCaseImplementation,
            LoadMessagesUseCaseError,
        },
        domain::chat_container::{
            ChatContainer,
            ChatContainerContext
        },
        infrastructure::{
            key_provider::KeyProviderError,
            chat_container_repository::ChatContainerRepositoryError,
            message_repository::MessageRepositoryError,
        },
    };
    use crate::helper::{
        chat_container_repository_stub::ChatContainerRepositoryStub,
        message_repository_stub::MessageRepositoryStub,
        local_storage_stub::LocalStorageStub,
        key_provider_stub::KeyProviderStub,
    };
    use std::sync::Arc;

    #[tokio::test]
    async fn successfully_load_messages() {
        let sender_public_key = "sender_public_key".to_string();
        let recipient_public_key = "recipient_public_key".to_string();
        let storage = Arc::new(LocalStorageStub { simulated_error: None });
        let provider = Arc::new(KeyProviderStub { simulated_error: None });
        let chat_container_id = "id";
        let chat_container = ChatContainer::new(
            chat_container_id.to_string(), 
            ChatContainerContext::Direct { other_public_key: recipient_public_key.clone() },
            vec![sender_public_key.clone(), recipient_public_key.clone()]
            );
        let container_repository = Arc::new(ChatContainerRepositoryStub::new(
            None,
            Some(chat_container),
        ));
        let message_repository = Arc::new(MessageRepositoryStub::new(None));
        let sut = LoadMessagesUseCaseImplementation::new(storage, provider, container_repository, message_repository);

        let result = sut.execute(chat_container_id.to_string()).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn unauthorized_failed_to_load_messages() {
        let sender_public_key = "sender_public_key".to_string();
        let recipient_public_key = "recipient_public_key".to_string();
        let storage_error = "Simulated Error".to_string();
        let storage = Arc::new(LocalStorageStub { simulated_error: Some(storage_error.clone()) });
        let provider = Arc::new(KeyProviderStub { simulated_error: None });
        let chat_container_id = "id";
        let chat_container = ChatContainer::new(
            chat_container_id.to_string(), 
            ChatContainerContext::Direct { other_public_key: recipient_public_key.clone() },
            vec![sender_public_key.clone(), recipient_public_key.clone()]
            );
        let container_repository = Arc::new(ChatContainerRepositoryStub::new(
            None,
            Some(chat_container),
        ));
        let message_repository = Arc::new(MessageRepositoryStub::new(None));
        let sut = LoadMessagesUseCaseImplementation::new(storage, provider, container_repository, message_repository);

        let result = sut.execute(chat_container_id.to_string()).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), LoadMessagesUseCaseError::Unauthorized(storage_error.clone()));
    }

    #[tokio::test]
    async fn invalid_key_failed_to_load_messages() {
        let sender_public_key = "sender_public_key".to_string();
        let recipient_public_key = "recipient_public_key".to_string();
        let storage = Arc::new(LocalStorageStub { simulated_error: None });
        let provider_error = KeyProviderError::InvalidKey("Invalid Key".to_string());
        let provider = Arc::new(KeyProviderStub { simulated_error: Some(provider_error.clone()) });
        let chat_container_id = "id";
        let chat_container = ChatContainer::new(
            chat_container_id.to_string(), 
            ChatContainerContext::Direct { other_public_key: recipient_public_key.clone() },
            vec![sender_public_key.clone(), recipient_public_key.clone()]
            );
        let container_repository = Arc::new(ChatContainerRepositoryStub::new(
            None,
            Some(chat_container),
        ));
        let message_repository = Arc::new(MessageRepositoryStub::new(None));
        let sut = LoadMessagesUseCaseImplementation::new(storage, provider, container_repository, message_repository);

        let result = sut.execute(chat_container_id.to_string()).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), LoadMessagesUseCaseError::InvalidKey(provider_error.clone()));
    }

    #[tokio::test]
    async fn container_repository_error_failed_to_load_messages() {
        let storage = Arc::new(LocalStorageStub { simulated_error: None });
        let provider = Arc::new(KeyProviderStub { simulated_error: None });
        let chat_container_id = "id";
        let container_repository_error = ChatContainerRepositoryError::ContainerNotFound;
        let container_repository = Arc::new(ChatContainerRepositoryStub::new(
            Some(container_repository_error.clone()),
            None,
        ));
        let message_repository = Arc::new(MessageRepositoryStub::new(None));
        let sut = LoadMessagesUseCaseImplementation::new(storage, provider, container_repository, message_repository);

        let result = sut.execute(chat_container_id.to_string()).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), LoadMessagesUseCaseError::ContainerRepositoryError(container_repository_error.clone()));
    }

    #[tokio::test]
    async fn message_repository_error_failed_to_load_messages() {
        let sender_public_key = "sender_public_key".to_string();
        let recipient_public_key = "recipient_public_key".to_string();
        let storage = Arc::new(LocalStorageStub { simulated_error: None });
        let provider = Arc::new(KeyProviderStub { simulated_error: None });
        let chat_container_id = "id";
        let chat_container = ChatContainer::new(
            chat_container_id.to_string(), 
            ChatContainerContext::Direct { other_public_key: recipient_public_key.clone() },
            vec![sender_public_key.clone(), recipient_public_key.clone()]
            );
        let container_repository = Arc::new(ChatContainerRepositoryStub::new(
            None,
            Some(chat_container),
        ));
        let message_repository_error = MessageRepositoryError::UnknownError("Unknown Error".to_string());
        let message_repository = Arc::new(MessageRepositoryStub::new(Some(message_repository_error.clone())));
        let sut = LoadMessagesUseCaseImplementation::new(storage, provider, container_repository, message_repository);

        let result = sut.execute(chat_container_id.to_string()).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), LoadMessagesUseCaseError::MessageRepositoryError(message_repository_error.clone()));
    }
}
