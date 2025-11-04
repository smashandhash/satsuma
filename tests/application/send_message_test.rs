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
    use crate::helper::nostr_event_validator_stub::NostrEventValidatorStub;
    use rstest::rstest;

    #[test]
    fn send_message_succeed() {
        let id = "id".to_string();
        let max_length = 200;
        let content = "Hello, Bob.";
        let validator = NostrEventValidatorStub { simulated_error: None };
        let sut = NostrSendMessageUseCase { max_length, validator };

        let message = Message::new(id, "".to_string(), content.to_string(), 0, 0, Vec::new(), "".to_string());
        let result = sut.execute(message);

        assert!(result.is_ok());
    }

    #[rstest]
    #[case("rejected for empty message", 200, "", SendMessageUseCaseError::EmptyMessage)]
    #[case("rejected for message has only spaces", 200, "   ", SendMessageUseCaseError::EmptyMessage)]
    #[case("rejected for message is too long", 8, "Hello, Bob", SendMessageUseCaseError::MessageTooLong)]
    fn send_message_error(
        #[case] _label: &str,
        #[case] max_length: usize,
        #[case] content: &str,
        #[case] expected_error: SendMessageUseCaseError) {
        let id = "id".to_string();
        let validator = NostrEventValidatorStub { simulated_error: None };
        let sut = NostrSendMessageUseCase { max_length, validator };

        let message = Message::new(id, "".to_string(), content.to_string(), 0, 0, Vec::new(), "".to_string());
        let result = sut.execute(message);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), expected_error);
    }

    #[test]
    fn send_message_nostr_error() {
        let simulated_error = NostrEventValidatorError::TimestampError(TimestampValidatorError::InvalidTimestamp);
        let id = "id".to_string();
        let max_length = 200;
        let content = "Hello, Bob.";
        let validator = NostrEventValidatorStub { simulated_error: Some(simulated_error.clone()) };
        let expected_error = SendMessageUseCaseError::NostrError(simulated_error.clone());
        let sut = NostrSendMessageUseCase { max_length, validator };

        let message = Message::new(id, "".to_string(), content.to_string(), 0, 0, Vec::new(), "".to_string());
        let result = sut.execute(message);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), expected_error);
    }
}
