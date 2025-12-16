use crate::{
    domain::user::User,
    infrastructure::{
        relay_publisher::RelayPublisher,
        relay_publisher::RelayPublisherError,
    },
};
use nostr_sdk::Metadata;
use std::sync::Arc;

pub trait ChangeNameUseCase {
    fn execute(&mut self, new_name: String) -> Result<(), ChangeNameUseCaseError>;
}

pub struct NostrChangeNameUseCase<'a, R: RelayPublisher> {
    pub user: &'a mut User,
    pub relay_publisher: Arc<R>,
}

impl<'a, R: RelayPublisher> NostrChangeNameUseCase<'a, R> {
    pub fn new(user: &'a mut User, relay_publisher: Arc<R>) -> Self {
        Self {
            user,
            relay_publisher
        }
    }
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
            .map_err(|e| ChangeNameUseCaseError::RelayFailed(e))?;

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum ChangeNameUseCaseError {
    InvalidName,
    RelayFailed(RelayPublisherError),
}
