use satsuma::domain::user::User;

#[cfg(test)]
mod tests {
    use super::*;
   
    #[test]
    fn init_user_does_nothing() {
        let user = User::new(1, "Alice");
    }

    #[test]
    fn user_can_change_name() {
        let mut user = User::new(1, "Alice");
        user.change_name("Alicia");
        assert_eq!(user.name, "Alicia");
    }
}
