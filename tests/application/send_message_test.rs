#[cfg(test)]
mod tests {
    use satsuma::{
        application::send_message::{
            SendMessageUseCase,
            SendMessageUseCaseError,
            NostrSendMessageUseCase,
        },
        domain::message::{
            Message,
            MessageKind
        }
    };
    use crate::helper::message_repository_stub::MessageRepositoryStub;
    use rstest::rstest;

    #[tokio::test]
    async fn send_message_succeed() {
        let id = "id".to_string();
        let content = "Hello, Bob.";
        let recipient_public_key_string = "npub001".to_string();
        let repository = MessageRepositoryStub::new(Vec::new());
        let sut = NostrSendMessageUseCase { repository };

        let message = Message::new(
            id, 
            "".to_string(), 
            content.to_string(), 
            0, 
            MessageKind::Direct(recipient_public_key_string), 
            Vec::new(), 
            "".to_string()
        );
        let result = sut.execute(&message).await;

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
        let id = "id".to_string();
        let recipient_public_key_string = "npub001".to_string();
        let repository = MessageRepositoryStub::new(Vec::new());
        let sut = NostrSendMessageUseCase { repository };

        let message = Message::new(
            id, 
            "".to_string(), 
            content.to_string(), 
            0, 
            MessageKind::Direct(recipient_public_key_string), 
            Vec::new(), 
            "".to_string()
        );
        let result = sut.execute(&message).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), expected_error);
    }
}
