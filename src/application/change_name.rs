use crate::domain::user::User;
use crate::infrastructure::{
    nostr_event::NostrEvent,
    relay_manager::RelayManager
};
use serde_json::json;

pub trait ChangeNameUseCase {
    fn execute(&mut self, new_name: String) -> Result<(), ChangeNameUseCaseError>;
}

pub struct NostrChangeNameUseCase<'a> {
    pub user: &'a mut User,
    pub relay_manager: &'a RelayManager,
}

impl<'a> ChangeNameUseCase for NostrChangeNameUseCase<'a> {
    fn execute(&mut self, new_name: String) -> Result<(), ChangeNameUseCaseError> {
        if new_name.trim().is_empty() {
            return Err(ChangeNameUseCaseError::InvalidName);
        }

        self.user.change_name(new_name.clone());

        let content = json!({
            "name": new_name
        }).to_string();
        let event = NostrEvent::new(0, content, &self.user.public_key);

        self.relay_manager
            .publish(event)
            .map_err(|e| ChangeNameUseCaseError::SaveFailed(e))?;

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum ChangeNameUseCaseError {
    InvalidName,
    SaveFailed(String),
}
