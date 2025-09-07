use satsuma::application::change_name::ChangeNameUseCase;
use satsuma::domain::user::User;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn change_name_with_new_name() {
        let mut user = User::new(1, "Alice");
        let use_case = ChangeNameUseCase::new();

        let result = use_case.execute(&mut user, "Bob");
        
        assert!(result.is_ok());
        assert_eq!(user.name, "Bob");
    }

    #[test]
    fn change_name_cannot_with_empty_name() {
        let mut user= User::new(1, "Alice");
        let use_case = ChangeNameUseCase::new();

        let result = use_case.execute(&mut user, "");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "New name cannot be empty");
    }
}
