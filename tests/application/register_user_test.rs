#[cfg(test)]
mod tests {
    use satsuma::application::register_user::{
        RegisterUserUseCase,
        NostrRegisterUserUseCase,
        RegisterUserUseCaseError
    };
    use satsuma::domain::user::User;
    use satsuma::infrastructure::{
        local_storage::LocalStorage,
        nostr_event::NostrEvent,
        relay_publisher::RelayPublisher,
        relay_publisher::RelayPublisherError
    };

    #[test]
    fn register_user_creates_new_user() {
        let local_storage = LocalStorageStub { simulated_error: None };
        let relay_publisher = RelayPublisherStub { simulated_error: None };
        let use_case = NostrRegisterUserUseCase { 
            storage: &local_storage, 
            relay_publisher: &relay_publisher 
        };
        let result = use_case.execute("Alice");
        
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.name, "Alice");
        assert!(user.public_key != "");
    }

    #[test]
    fn register_multiple_users_gets_unique_ids() {
        let local_storage = LocalStorageStub { simulated_error: None };
        let relay_publisher = RelayPublisherStub { simulated_error: None };
        let use_case = NostrRegisterUserUseCase { 
            storage: &local_storage,
            relay_publisher: &relay_publisher
        };
        let alice_result = use_case.execute("Alice");
        let bob_result = use_case.execute("Bob");
        let alice = alice_result.unwrap();
        let bob = bob_result.unwrap();

        assert_ne!(alice.public_key, bob.public_key);
    }

    #[test]
    fn cannot_register_with_empty_name() {
        let local_storage = LocalStorageStub { simulated_error: None };
        let relay_publisher = RelayPublisherStub { simulated_error: None };
        let use_case = NostrRegisterUserUseCase { 
            storage: &local_storage,
            relay_publisher: &relay_publisher
        };
        let result = use_case.execute("");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), RegisterUserUseCaseError::InvalidName);
    }

    #[test]
    fn trims_name_on_registration() {
        let local_storage = LocalStorageStub { simulated_error: None };
        let relay_publisher = RelayPublisherStub { simulated_error: None };
        let use_case = NostrRegisterUserUseCase { 
            storage: &local_storage,
            relay_publisher: &relay_publisher
        };
        let result = use_case.execute("  Alice  ");
        let user = result.unwrap();
        assert_eq!(user.name, "Alice");
    }

    #[test]
    fn local_storage_failing() {
        let error_text = "Failed for test".to_string();
        let local_storage = LocalStorageStub { simulated_error: Some(error_text.clone()) };
        let relay_publisher = RelayPublisherStub { simulated_error: None };
        let use_case = NostrRegisterUserUseCase { 
            storage: &local_storage,
            relay_publisher: &relay_publisher
        };
        let result = use_case.execute("Bob");
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), RegisterUserUseCaseError::PersistError(error_text.clone()));
    }

    #[test]
    fn relay_publisher_failing() {
        let error = RelayPublisherError::ConnectionFailed;
        let local_storage = LocalStorageStub { simulated_error: None };
        let relay_publisher = RelayPublisherStub { simulated_error: Some(error.clone()) };
        let use_case = NostrRegisterUserUseCase {
            storage: &local_storage,
            relay_publisher: &relay_publisher
        };
        let result = use_case.execute("Alisa");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), RegisterUserUseCaseError::RelayFailed(error.clone()));
    }

    pub struct LocalStorageStub {
        simulated_error: Option<String>
    }

    impl LocalStorage for LocalStorageStub {
        fn save_user(&self, _user: &User) -> Result<(), String> {
            if let Some(simulated_error) = &self.simulated_error {
                Err(simulated_error.clone())
            } else {
                Ok(())
            }
        }
    }

    pub struct RelayPublisherStub {
        simulated_error: Option<RelayPublisherError>
    }

    impl RelayPublisher for RelayPublisherStub {
        fn publish(&self, _event: NostrEvent) -> Result<(), RelayPublisherError> {
            self.simulated_error.clone().map_or(Ok(()), Err)
        }
    }
}
