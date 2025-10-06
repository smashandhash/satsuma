use crate::domain::user::User;
use crate::infrastructure::{
    nostr_event::NostrEvent,
    relay_publisher::RelayPublisher,
    relay_publisher::RelayPublisherError
};
use serde_json::json;

pub trait ChangeNameUseCase {
    fn execute(&mut self, new_name: String) -> Result<(), ChangeNameUseCaseError>;
}

pub struct NostrChangeNameUseCase<'a, R: RelayPublisher> {
    pub user: &'a mut User,
    pub relay_publisher: &'a R,
}

impl<'a, R: RelayPublisher> ChangeNameUseCase for NostrChangeNameUseCase<'a, R> {
    fn execute(&mut self, new_name: String) -> Result<(), ChangeNameUseCaseError> {
        if new_name.trim().is_empty() {
            return Err(ChangeNameUseCaseError::InvalidName);
        }

        self.user.change_name(new_name.clone());

        let content = json!({
            "name": new_name
        }).to_string();
        let event = NostrEvent::new(0, content, &self.user.public_key);

        self.relay_publisher
            .publish(event)
            .map_err(|e| ChangeNameUseCaseError::SaveFailed(e))?;

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum ChangeNameUseCaseError {
    InvalidName,
    SaveFailed(RelayPublisherError),
}
