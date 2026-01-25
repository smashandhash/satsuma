use crate::domain::user::User;

pub trait UserRepository {
    fn load(&self, public_key: String) -> Result<User, UserRepositoryError>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserRepositoryError {
    UserNotFound
}
