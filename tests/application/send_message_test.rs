#[cfg(test)]
mod tests {
    use satsuma::application::send_message::{
        SendMessageUseCase,
        SendMessageUseCaseError,
        NostrSendMessageUseCase,
    };
    use crate::helper::{
        key_provider_stub::KeyProviderStub,
        message_repository_stub::MessageRepositoryStub,
        local_storage_stub::LocalStorageStub
    };
    use rstest::rstest;

    #[tokio::test]
    async fn send_message_succeed() {
        let content = "Hello, Bob.";
        let recipient_public_key = "npub001".to_string();
        let provider = KeyProviderStub { simulated_error: None };
        let repository = MessageRepositoryStub::new(None, Vec::new());
        let storage = LocalStorageStub { simulated_error: None };
        let sut = NostrSendMessageUseCase { provider, repository, storage };

        let result = sut.execute(content.to_string(), Some(recipient_public_key), None, None, None).await;

        assert!(result.is_ok());
    }

    #[rstest]
    #[case("rejected for empty message", "", SendMessageUseCaseError::EmptyMessage)]
    #[case("rejected for message has only spaces", "   ", SendMessageUseCaseError::EmptyMessage)]
    #[tokio::test]
    async fn send_message_error(
        #[case] _label: &str,
        #[case] content: &str,
        #[case] expected_error: SendMessageUseCaseError) {
        let recipient_public_key = "npub001".to_string();
        let provider = KeyProviderStub { simulated_error: None };
        let repository = MessageRepositoryStub::new(None, Vec::new());
        let storage = LocalStorageStub { simulated_error: None };
        let sut = NostrSendMessageUseCase { provider, repository, storage };

        let result = sut.execute(content.to_string(), Some(recipient_public_key), None, None, None).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), expected_error);
    }
}
