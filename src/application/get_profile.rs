use crate::{
    domain::user::User,
    infrastructure::profile_repository::{
        ProfileRepository,
        ProfileRepositoryError,
    },
};

pub trait GetProfileUseCase {
    fn execute(&self, public_key: String) -> Result<User, GetProfileUseCaseError>;
}

pub struct GetProfileUseCaseImplementation<R: ProfileRepository> {
    pub repository: R,
}

impl<R: ProfileRepository> GetProfileUseCaseImplementation<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: ProfileRepository> GetProfileUseCase for GetProfileUseCaseImplementation<R> {
    fn execute(&self, public_key: String) -> Result<User, GetProfileUseCaseError> {
        let user = self.repository.load(public_key).map_err(|e| GetProfileUseCaseError::ProfileRepositoryError(e))?;

        Ok(user)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GetProfileUseCaseError {
    ProfileRepositoryError(ProfileRepositoryError)
}
