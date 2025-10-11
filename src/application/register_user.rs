use crate::domain::user::User;
use crate::infrastructure::{
    local_storage::LocalStorage,
    nostr_event::NostrEvent,
    relay_publisher::RelayPublisher,
    relay_publisher::RelayPublisherError
};
use serde_json::json;

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

        let user = User::new(&format!("npub{}", trimmed_desired_name), &trimmed_desired_name);

        self.storage.save_user(&user).map_err(|e| RegisterUserUseCaseError::PersistError(e))?;

        let content = json!({
            "name": user.name,
            "about": user.about,
            "picture": user.picture
        }).to_string();

        let event = NostrEvent::new(0, content, &user.public_key);

        self.relay_publisher
            .publish(event)
            .map_err(|e| RegisterUserUseCaseError::RelayFailed(e))?;
        
        Ok(user)
    }
}

#[derive(Debug, PartialEq)]
pub enum RegisterUserUseCaseError {
    InvalidName,
    PersistError(String),
    RelayFailed(RelayPublisherError)
}
