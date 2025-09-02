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
}
