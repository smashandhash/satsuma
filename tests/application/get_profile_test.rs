#[cfg(test)]
mod tests {
    use satsuma::domain::user::User;

    pub trait GetProfileUseCase {
        fn execute(&self, profile_id: String) -> User;
    }

    pub struct GetProfileUseCaseImplementation;

    impl GetProfileUseCase for GetProfileUseCaseImplementation {
        fn execute(&self, _profile_id: String) -> User {
            User::new("public_key", "name")
        }
    }

    #[test]
    fn get_profile() {
        let profile_id = "profile_id".to_string();
        let sut = GetProfileUseCaseImplementation;
        let result = sut.execute(profile_id);
        let expected_user = User::new("public_key", "name");

        assert_eq!(result, expected_user);
    }
}
