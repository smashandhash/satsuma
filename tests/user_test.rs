use satsuma::domain::user::User;

#[cfg(test)]
mod tests {
    use super::*;
   
    #[test]
    fn init_user_does_nothing() {
        let user = User::new(1, "Alice");
    }
}
