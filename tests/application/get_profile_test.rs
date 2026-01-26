#[cfg(test)]
mod tests {
    use satsuma::{
        application::get_profile::{
            GetProfileUseCase,
            GetProfileUseCaseImplementation,
            GetProfileUseCaseError,
        },
        domain::user::User,
        infrastructure::user_repository::UserRepositoryError,
    };
    use crate::helper::user_repository_stub::UserRepositoryStub;
    use rstest::rstest;
    use std::sync::Arc;

    #[rstest]
    #[case("Success", None)]
    #[case("Failed", Some(UserRepositoryError::UserNotFound))]
    fn get_user_based_on_repository(
        #[case] _label: &str,
        #[case] simulated_error: Option<UserRepositoryError>,
        ) {
        let repository = Arc::new(UserRepositoryStub::new(simulated_error.clone()));
        let public_key = "public_key".to_string();
        let sut = GetProfileUseCaseImplementation::new(repository);
        let result = sut.execute(public_key);
        let expected_user = User::new("public_key", "name");

        if let Some(simulated_error) = simulated_error {
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), GetProfileUseCaseError::UserRepositoryError(simulated_error));
        } else {
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected_user);
        }
    }
}
