#[cfg(test)]
mod tests {
    use satsuma::application::register_user::{
        RegisterUserUseCase,
        NostrRegisterUserUseCase,
        RegisterUserUseCaseError
    };
    use satsuma::domain::user::User;
    use satsuma::infrastructure::local_storage::LocalStorage;

    #[test]
    fn register_user_creates_new_user() {
        let local_storage = LocalStorageStub { simulated_error: None };
        let use_case = NostrRegisterUserUseCase { storage: &local_storage };
        let result = use_case.execute("Alice");
        
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.name, "Alice");
        assert!(user.public_key != "");
    }

    #[test]
    fn register_multiple_users_gets_unique_ids() {
        let local_storage = LocalStorageStub { simulated_error: None };
        let use_case = NostrRegisterUserUseCase { storage: &local_storage };
        let alice_result = use_case.execute("Alice");
        let bob_result = use_case.execute("Bob");
        let alice = alice_result.unwrap();
        let bob = bob_result.unwrap();

        assert_ne!(alice.public_key, bob.public_key);
    }

    #[test]
    fn cannot_register_with_empty_name() {
        let local_storage = LocalStorageStub { simulated_error: None };
        let use_case = NostrRegisterUserUseCase { storage: &local_storage };
        let result = use_case.execute("");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), RegisterUserUseCaseError::InvalidName);
    }

    #[test]
    fn trims_name_on_registration() {
        let local_storage = LocalStorageStub { simulated_error: None };
        let use_case = NostrRegisterUserUseCase { storage: &local_storage };
        let result = use_case.execute("  Alice  ");
        let user = result.unwrap();
        assert_eq!(user.name, "Alice");
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
}
