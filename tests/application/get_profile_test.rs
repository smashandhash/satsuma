#[cfg(test)]
mod tests {
    use satsuma::domain::user::User;

    pub trait GetProfileUseCase {
        fn execute(&self, profile_id: String) -> Result<User, GetProfileUseCaseError>;
    }

    pub struct GetProfileUseCaseImplementation;

    impl GetProfileUseCase for GetProfileUseCaseImplementation {
        fn execute(&self, profile_id: String) -> Result<User, GetProfileUseCaseError> {
            if profile_id == "not_found_id".to_string() {
                return Err(GetProfileUseCaseError::ProfileNotFound)
            }
            Ok(User::new("public_key", "name"))
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum GetProfileUseCaseError {
        ProfileNotFound
    }

    #[test]
    fn get_profile_success() {
        let profile_id = "profile_id".to_string();
        let sut = GetProfileUseCaseImplementation;
        let result = sut.execute(profile_id);
        let expected_user = User::new("public_key", "name");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_user);
    }

    #[test]
    fn get_profile_failed() {
        let profile_id = "not_found_id".to_string();
        let sut = GetProfileUseCaseImplementation;
        let result = sut.execute(profile_id);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GetProfileUseCaseError::ProfileNotFound);
    }
}
