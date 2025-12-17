#[cfg(test)]
mod tests {
    use satsuma::{
        application::send_message::{
            SendMessageUseCase,
            SendMessageUseCaseError,
            NostrSendMessageUseCase,
        },
        domain::chat_container::ChatContainerContext,
    };
    use crate::helper::{
        key_provider_stub::KeyProviderStub,
        message_repository_stub::MessageRepositoryStub,
        local_storage_stub::LocalStorageStub
    };
    use rstest::rstest;
    use std::sync::Arc;

    #[tokio::test]
    async fn send_message_succeed() {
        let content = "Hello, Bob.";
        let provider = Arc::new(KeyProviderStub::new(None));
        let repository = Arc::new(MessageRepositoryStub::new(None));
        let storage = Arc::new(LocalStorageStub::new(None));
        let sut = NostrSendMessageUseCase::new(provider, repository, storage);

        let result = sut.execute(content.to_string(), "session_id".to_string(), ChatContainerContext::Direct { other_public_key: "other_public_key".to_string() }, None).await;

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
        let provider = Arc::new(KeyProviderStub::new(None));
        let repository = Arc::new(MessageRepositoryStub::new(None));
        let storage = Arc::new(LocalStorageStub::new(None));
        let sut = NostrSendMessageUseCase::new(provider, repository, storage);

        let result = sut.execute(content.to_string(), "session_id".to_string(), ChatContainerContext::Direct { other_public_key: "other_public_key".to_string() }, None).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), expected_error);
    }
}
