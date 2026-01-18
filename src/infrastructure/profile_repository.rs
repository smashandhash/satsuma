use crate::domain::user::User;

pub trait ProfileRepository {
    fn load(&self, public_key: String) -> Result<User, ProfileRepositoryError>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProfileRepositoryError {
    ProfileNotFound
}
