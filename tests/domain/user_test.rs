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
        let new_name = "Alicia".to_string();
        user.change_name(new_name.clone());
        assert_eq!(user.name, new_name);
    }

    #[test]
    fn changing_name_does_not_affect_id() {
        let mut user = User::new(1, "Alice");
        let new_name = "Alicia".to_string();
        user.change_name(new_name.clone());
        assert_eq!(user.id, 1);
    }
}
