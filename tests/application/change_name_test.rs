#[cfg(test)]
mod tests {
    use satsuma::{
        application::change_name::ChangeNameUseCase,
        application::change_name::NostrChangeNameUseCase,
        domain::user::User,
        infrastructure::relay_publisher::RelayPublisherError
    };
    use crate::helper::relay_publisher_stub::RelayPublisherStub;
    use rstest::rstest;
    use std::sync::Arc;

    #[rstest]
    #[case("change name with a new name", "Bob", true)]
    #[case("change name with an empty name", "", false)]
    #[case("change name with spaces on its name", " Bob ", true)]
    fn change_name(
        #[case] _label: &str,
        #[case] new_name: String,
        #[case] should_succeed: bool
        ) {
        let mut user = User::new("npub1234".into(), "Alice".into());
        let relay_publisher = Arc::new(RelayPublisherStub::new(None));
        let mut use_case = NostrChangeNameUseCase::new(&mut user, relay_publisher);

        let result = use_case.execute(new_name);
        
        assert_eq!(result.is_ok(), should_succeed);
    }

    #[rstest]
    #[case("connection failed", RelayPublisherError::ConnectionFailed)]
    #[case("unknown", RelayPublisherError::Unknown("Something wrong".to_string()))]
    fn error_handling(
        #[case] _label: &str,
        #[case] error: RelayPublisherError
    ) {
        let mut user = User::new("npub1", "Alice");
        let relay_publisher = Arc::new(RelayPublisherStub::new(Some(error)));
        let mut use_case = NostrChangeNameUseCase::new(&mut user, relay_publisher);

        let result = use_case.execute("Alisa".to_string());

        assert!(result.is_err());
    }
}
