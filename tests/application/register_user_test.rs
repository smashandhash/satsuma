#[cfg(test)]
mod tests {
    use satsuma::application::register_user::{
        RegisterUserUseCase,
        NostrRegisterUserUseCase,
        RegisterUserUseCaseError
    };
    use satsuma::infrastructure::relay_publisher::RelayPublisherError;
    use crate::helper::{
        relay_publisher_stub::RelayPublisherStub,
        local_storage_stub::LocalStorageStub
    };
    use std::sync::Arc;

    #[test]
    fn register_user_creates_new_user() {
        let use_case = make_sut(None, None);
        let result = use_case.execute("Alice");
        
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.name, "Alice");
        assert!(user.public_key != "");
    }

    #[test]
    fn register_multiple_users_gets_unique_ids() {
        let use_case = make_sut(None, None);
        let alice_result = use_case.execute("Alice");
        let bob_result = use_case.execute("Bob");
        let alice = alice_result.unwrap();
        let bob = bob_result.unwrap();

        assert_ne!(alice.public_key, bob.public_key);
    }

    #[test]
    fn cannot_register_with_empty_name() {
        let use_case = make_sut(None, None);
        let result = use_case.execute("");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), RegisterUserUseCaseError::InvalidName);
    }

    #[test]
    fn trims_name_on_registration() {
        let use_case = make_sut(None, None);
        let result = use_case.execute("  Alice  ");
        let user = result.unwrap();
        assert_eq!(user.name, "Alice");
    }

    #[test]
    fn local_storage_failing() {
        let error_text = "Failed for test".to_string();
        let use_case = make_sut(Some(error_text.clone()), None);
        let result = use_case.execute("Bob");
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), RegisterUserUseCaseError::PersistError(error_text.clone()));
    }

    #[test]
    fn relay_publisher_failing() {
        let error = RelayPublisherError::ConnectionFailed;
        let use_case = make_sut(None, Some(error.clone()));
        let result = use_case.execute("Alisa");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), RegisterUserUseCaseError::RelayFailed(error.clone()));
    }

    fn make_sut(local_storage_error: Option<String>, relay_publisher_error: Option<RelayPublisherError>) -> NostrRegisterUserUseCase<LocalStorageStub, RelayPublisherStub> {
        NostrRegisterUserUseCase::new(
            Arc::new(LocalStorageStub::new(local_storage_error)),
            Arc::new(RelayPublisherStub::new(relay_publisher_error)),
        )
    }
}
