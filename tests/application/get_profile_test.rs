#[cfg(test)]
mod tests {
    use satsuma::{
        application::get_profile::{
            GetProfileUseCase,
            GetProfileUseCaseImplementation,
            GetProfileUseCaseError,
        },
        domain::user::User,
        infrastructure::profile_repository::ProfileRepositoryError,
    };
    use crate::helper::profile_repository_stub::ProfileRepositoryStub;
    use rstest::rstest;

    #[rstest]
    #[case("Success", None)]
    #[case("Failed", Some(ProfileRepositoryError::ProfileNotFound))]
    fn get_profile_based_on_repository(
        #[case] _label: &str,
        #[case] simulated_error: Option<ProfileRepositoryError>,
        ) {
        let repository = ProfileRepositoryStub::new(simulated_error.clone());
        let public_key = "public_key".to_string();
        let sut = GetProfileUseCaseImplementation::new(repository);
        let result = sut.execute(public_key);
        let expected_user = User::new("public_key", "name");

        if let Some(simulated_error) = simulated_error {
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), GetProfileUseCaseError::ProfileRepositoryError(simulated_error));
        } else {
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected_user);
        }
    }
}
