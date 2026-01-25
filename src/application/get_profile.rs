use crate::{
    domain::user::User,
    infrastructure::user_repository::{
        UserRepository,
        UserRepositoryError,
    },
};

pub trait GetProfileUseCase {
    fn execute(&self, public_key: String) -> Result<User, GetProfileUseCaseError>;
}

pub struct GetProfileUseCaseImplementation<R: UserRepository> {
    pub repository: R,
}

impl<R: UserRepository> GetProfileUseCaseImplementation<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: UserRepository> GetProfileUseCase for GetProfileUseCaseImplementation<R> {
    fn execute(&self, public_key: String) -> Result<User, GetProfileUseCaseError> {
        let user = self.repository.load(public_key).map_err(|e| GetProfileUseCaseError::UserRepositoryError(e))?;

        Ok(user)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GetProfileUseCaseError {
    UserRepositoryError(UserRepositoryError)
}
