use crate::domain::user::User;
use crate::infrastructure::local_storage::LocalStorage;

pub trait RegisterUserUseCase {
    fn execute(&self, desired_name: &str) -> Result<User, RegisterUserUseCaseError>;
}

pub struct NostrRegisterUserUseCase<'a, S: LocalStorage> {
    pub storage: &'a S,
}

impl<'a, S> RegisterUserUseCase for NostrRegisterUserUseCase<'a, S> where S: LocalStorage {
    fn execute(&self, desired_name: &str) -> Result<User, RegisterUserUseCaseError> {
        let trimmed_desired_name = desired_name.trim();
        if trimmed_desired_name.is_empty() {
            return Err(RegisterUserUseCaseError::InvalidName);
        }

        let user = User::new(&format!("npub{}", trimmed_desired_name), &trimmed_desired_name);

        self.storage.save_user(&user).map_err(|e| RegisterUserUseCaseError::PersistError(e))?;
        
        Ok(user)
    }
}

#[derive(Debug, PartialEq)]
pub enum RegisterUserUseCaseError {
    InvalidName,
    PersistError(String),
}
