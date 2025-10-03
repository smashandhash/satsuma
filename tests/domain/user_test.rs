#[cfg(test)]
mod tests {
    use satsuma::domain::user::User;
   
    #[test]
    fn init_user_does_nothing() {
        let _user = User::new(1, "Alice");
    }

    #[test]
    fn user_can_change_name() {
        let mut user = User::new(1, "Alice");
        user.change_name("Alicia");
        assert_eq!(user.name, "Alicia");
    }

    #[test]
    fn changing_name_does_not_affect_id() {
        let mut user = User::new(1, "Alice");
        user.change_name("Alicia");
        assert_eq!(user.id, 1);
    }
}
