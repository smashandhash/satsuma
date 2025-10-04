use crate::domain::user::User;

pub trait ChangeNameUseCase {
    fn execute(&mut self, new_name: String) -> Result<(), ChangeNameUseCaseError>;
}

pub struct NostrChangeNameUseCase {
    pub user: User,
}

impl ChangeNameUseCase for NostrChangeNameUseCase {
    fn execute(&mut self, new_name: String) -> Result<(), ChangeNameUseCaseError> {
        if new_name.trim().is_empty() {
            return Err(ChangeNameUseCaseError::InvalidName);
        }

        self.user.change_name(new_name);
        Ok(())
    }
}

#[derive(Debug)]
pub enum ChangeNameUseCaseError {
    InvalidName,
    SaveFailed(String),
}
