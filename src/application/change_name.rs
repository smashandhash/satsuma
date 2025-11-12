use crate::domain::user::User;
use crate::infrastructure::{
    relay_publisher::RelayPublisher,
    relay_publisher::RelayPublisherError
};
use nostr_sdk::Metadata;

pub trait ChangeNameUseCase {
    fn execute(&mut self, new_name: String) -> Result<(), ChangeNameUseCaseError>;
}

pub struct NostrChangeNameUseCase<'a, R: RelayPublisher> {
    pub user: &'a mut User,
    pub relay_publisher: R,
}

impl<'a, R: RelayPublisher> ChangeNameUseCase for NostrChangeNameUseCase<'a, R> {
    fn execute(&mut self, new_name: String) -> Result<(), ChangeNameUseCaseError> {
        if new_name.trim().is_empty() {
            return Err(ChangeNameUseCaseError::InvalidName);
        }

        self.user.change_name(new_name.clone());

        let metadata = Metadata::new().name(new_name);
        self.relay_publisher
            .publish(&metadata)
            .map_err(|e| ChangeNameUseCaseError::SaveFailed(e))?;

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum ChangeNameUseCaseError {
    InvalidName,
    SaveFailed(RelayPublisherError),
}
