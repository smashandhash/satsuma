use satsuma::application::register_user::RegisterUserUseCase;
use satsuma::domain::user::User;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_user_creates_new_user() {
        let mut use_case = RegisterUserUseCase::new();
        let user = use_case.execute("Alice".to_string());

        assert_eq!(user.name, "Alice");
        assert!(user.id > 0);
    }

    #[test]
    fn register_multiple_users_gets_unique_ids() {
        let mut use_case = RegisterUserUseCase::new();
        let alice = use_case.execute("Alice".to_string());
        let bob = use_case.execute("Bob".to_string());

        assert_ne!(alice.id, bob.id);
    }
}
