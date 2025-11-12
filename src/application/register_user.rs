use crate::domain::user::User;
use crate::infrastructure::{
    local_storage::LocalStorage,
    relay_publisher::RelayPublisher,
    relay_publisher::RelayPublisherError
};
use nostr_sdk::prelude::*;

pub trait RegisterUserUseCase {
    fn execute(&self, desired_name: &str) -> Result<User, RegisterUserUseCaseError>;
}

pub struct NostrRegisterUserUseCase<S: LocalStorage, R: RelayPublisher> {
    pub storage: S,
    pub relay_publisher: R
}

impl<S, R> RegisterUserUseCase for NostrRegisterUserUseCase<S, R> where S: LocalStorage, R: RelayPublisher {
    fn execute(&self, desired_name: &str) -> Result<User, RegisterUserUseCaseError> {
        let trimmed_desired_name = desired_name.trim();
        if trimmed_desired_name.is_empty() {
            return Err(RegisterUserUseCaseError::InvalidName);
        }

        let keys = Keys::generate();
        let public_key = keys.public_key().to_bech32().unwrap();
        
        let user = User::new(&public_key, &trimmed_desired_name);
        self.storage.save_user(&user).map_err(|e| RegisterUserUseCaseError::PersistError(e))?;

        // TODO: Save secret key to the storage later
        
        let metadata = Metadata::new().name(trimmed_desired_name).about("New to Nostr");
        self.relay_publisher
            .publish(&metadata)
            .map_err(|e| RegisterUserUseCaseError::RelayFailed(e))?;

        Ok(user)
    }
}

#[derive(Debug, PartialEq)]
pub enum RegisterUserUseCaseError {
    InvalidName,
    PersistError(String),
    EventError(String),
    RelayFailed(RelayPublisherError)
}
