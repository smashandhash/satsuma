#[cfg(test)]
mod tests {
    use satsuma::{
        application::send_message::{
            SendMessageUseCase,
            SendMessageUseCaseError,
            NostrSendMessageUseCase,
        },
        domain::{
            message::Message,
            services::{
                nostr_event_validator::NostrEventValidatorError,
                timestamp_validator::TimestampValidatorError
            }
        }
    };
    use crate::helper::{
        nostr_event_validator_stub::NostrEventValidatorStub,
        message_repository_stub::MessageRepositoryStub
    };
    use rstest::rstest;

    #[tokio::test]
    async fn send_message_succeed() {
        let id = "id".to_string();
        let content = "Hello, Bob.";
        let validator = NostrEventValidatorStub { simulated_error: None };
        let repository = MessageRepositoryStub::new(Vec::new());
        let sut = NostrSendMessageUseCase { validator, repository };

        let message = Message::new(id, "".to_string(), content.to_string(), 0, 0, Vec::new(), "".to_string());
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
        let validator = NostrEventValidatorStub { simulated_error: None };
        let repository = MessageRepositoryStub::new(Vec::new());
        let sut = NostrSendMessageUseCase { validator, repository };

        let message = Message::new(id, "".to_string(), content.to_string(), 0, 0, Vec::new(), "".to_string());
        let result = sut.execute(&message).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), expected_error);
    }

    #[tokio::test]
    async fn send_message_nostr_error() {
        let simulated_error = NostrEventValidatorError::TimestampError(TimestampValidatorError::InvalidTimestamp);
        let id = "id".to_string();
        let content = "Hello, Bob.";
        let validator = NostrEventValidatorStub { simulated_error: Some(simulated_error.clone()) };
        let repository = MessageRepositoryStub::new(Vec::new());
        let expected_error = SendMessageUseCaseError::NostrError(simulated_error.clone());
        let sut = NostrSendMessageUseCase { validator, repository };

        let message = Message::new(id, "".to_string(), content.to_string(), 0, 0, Vec::new(), "".to_string());
        let result = sut.execute(&message).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), expected_error);
    }
}
