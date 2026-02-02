use crate::domain::user::User;

pub trait UserRepository {
    fn load(&self, public_key: String) -> Result<User, UserRepositoryError>;
    fn change_role(&self, group_id: String, assigner_public_key: String, target_public_key: String, roles: Vec<String>, previous_event_id: Option<String>) -> Result<(), UserRepositoryError>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserRepositoryError {
    UserNotFound,
    FailedToChangeUserRole
}
