#[cfg(test)]
mod tests {
    use satsuma::{
        application::change_name::ChangeNameUseCase,
        application::change_name::NostrChangeNameUseCase,
        domain::user::User,
        infrastructure::relay_manager::RelayManager,
    };
    use rstest::rstest;

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
        let relay_manager = RelayManager;
        let mut use_case = NostrChangeNameUseCase { user: &mut user, relay_manager: &relay_manager };

        let result = use_case.execute(new_name);
        
        assert_eq!(result.is_ok(), should_succeed);
    }
}
