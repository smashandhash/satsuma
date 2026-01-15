#[cfg(test)]
mod tests {
    use satsuma::domain::user::User;

    pub trait GetProfileUseCase {
        fn execute(&self, public_key: String) -> Result<User, GetProfileUseCaseError>;
    }

    pub struct GetProfileUseCaseImplementation<R: ProfileRepository> {
        pub repository: R,
    }

    impl<R: ProfileRepository> GetProfileUseCaseImplementation<R> {
        fn new(repository: R) -> Self {
            Self { repository }
        }
    }

    impl<R: ProfileRepository> GetProfileUseCase for GetProfileUseCaseImplementation<R> {
        fn execute(&self, public_key: String) -> Result<User, GetProfileUseCaseError> {
            let user = self.repository.load(public_key).map_err(|e| GetProfileUseCaseError::ProfileRepositoryError(e))?;

            Ok(user)
        }
    }

    pub trait ProfileRepository {
        fn load(&self, public_key: String) -> Result<User, ProfileRepositoryError>;
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum GetProfileUseCaseError {
        ProfileRepositoryError(ProfileRepositoryError)
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum ProfileRepositoryError {
        ProfileNotFound
    }

    #[test]
    fn get_profile_success() {
        let repository = ProfileRepositoryStub::new(None);
        let public_key = "public_key".to_string();
        let sut = GetProfileUseCaseImplementation::new(repository);
        let result = sut.execute(public_key);
        let expected_user = User::new("public_key", "name");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_user);
    }

    #[test]
    fn get_profile_failed() {
        let repository_error = ProfileRepositoryError::ProfileNotFound;
        let repository = ProfileRepositoryStub::new(Some(repository_error.clone()));
        let public_key = "not_found_id".to_string();
        let sut = GetProfileUseCaseImplementation::new(repository);
        let result = sut.execute(public_key);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GetProfileUseCaseError::ProfileRepositoryError(repository_error));
    }

    pub struct ProfileRepositoryStub {
        pub simulated_error: Option<ProfileRepositoryError>,
    }

    impl ProfileRepositoryStub {
        fn new(simulated_error: Option<ProfileRepositoryError>) -> Self {
            Self { simulated_error }
        }
    }

    impl ProfileRepository for ProfileRepositoryStub {
        fn load(&self, _public_key: String) -> Result<User, ProfileRepositoryError> {
            self.simulated_error.clone().map_or(Ok(User::new("public_key", "name")), Err)
        }
    }
}
