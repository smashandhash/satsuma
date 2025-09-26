#[cfg(test)]
mod tests {
    use satsuma::application::change_name::ChangeNameUseCase;
    use satsuma::domain::user::User;
    use rstest::rstest;

    #[rstest]
    #[case("change name with a new name", "Bob", true)]
    #[case("change name with an empty name", "", false)]
    #[case("change name with spaces on its name", " Bob ", true)]
    fn change_name(
        #[case] _label: &str,
        #[case] new_name: &str,
        #[case] should_succeed: bool
        ) {
        let mut user = User::new(1, "Alice");
        let use_case = ChangeNameUseCase;

        let result = use_case.execute(&mut user, &new_name);
        
        assert_eq!(result.is_ok(), should_succeed);
    }
}
