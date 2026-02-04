use crate::domain::user::User;

pub trait UserRepository {
    fn load(&self, public_key: String) -> Result<User, UserRepositoryError>;
    fn change_role(&self, group_id: String, assigner_public_key: String, target_public_key: String, roles: Vec<String>, previous_event_id: Option<String>) -> Result<(), UserRepositoryError>;
}

pub struct NostrUserRepository;

impl UserRepository for NostrUserRepository {
    fn load(&self, _public_key: String) -> Result<User, UserRepositoryError> {
        return Err(UserRepositoryError::UserNotFound)
    }

    fn change_role(&self, _group_id: String, _assigner_public_key: String, _target_public_key: String, _roles: Vec<String>, _previous_event_id: Option<String>) -> Result<(), UserRepositoryError> {
        return Err(UserRepositoryError::FailedToChangeUserRole)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserRepositoryError {
    UserNotFound,
    FailedToChangeUserRole
}

// Note for change_role implementation
// TODO: Set the user's public key who do this thing
// TODO: Kind is 9000
// TODO: Create tags variable with 3 properties
// TODO: 1. "h" for group_id
// TODO: 2. "p" for target's public key
// TODO: 3. "previous" for an optional previous event_id, but it's recommended to have it.
// TODO: Set the content into fixed value of "Modify a participant of `target's public_key` into a role of `target_role`" 
