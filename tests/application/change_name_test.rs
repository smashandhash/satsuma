use satsuma::application::change_name::ChangeNameUseCase;
use satsuma::domain::user::User;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn change_name_with_new_name() {
        let mut user = User::new(1, "Alice");
        let use_case = ChangeNameUseCase::new();

        use_case.execute(&mut user, "Bob");

        assert_eq!(user.name, "Bob");
    }
}
