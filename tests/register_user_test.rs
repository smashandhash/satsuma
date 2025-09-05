use satsuma::application::register_user::RegisterUserUseCase;
use satsuma::domain::user::User;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_user_creates_new_user() {
        let mut use_case = RegisterUserUseCase::new();
        let result = use_case.execute("Alice".to_string());
        
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.name, "Alice");
        assert!(user.id > 0);
    }

    #[test]
    fn register_multiple_users_gets_unique_ids() {
        let mut use_case = RegisterUserUseCase::new();
        let alice_result = use_case.execute("Alice".to_string());
        let bob_result = use_case.execute("Bob".to_string());
        let alice = alice_result.unwrap();
        let bob = bob_result.unwrap();

        assert_ne!(alice.id, bob.id);
    }

    #[test]
    fn cannot_register_with_empty_name() {
        let mut use_case = RegisterUserUseCase::new();
        let result = use_case.execute("".to_string());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "User name cannot be empty");
    }

    #[test]
    fn trims_name_on_registration() {
        let mut use_case = RegisterUserUseCase::new();
        let result = use_case.execute("  Alice  ".to_string());
        let user = result.unwrap();
        assert_eq!(user.name, "Alice");
    }
}
