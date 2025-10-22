use sha2::{Sha256, Digest};
use serde_json::json;
use crate::domain::event::Event;

pub trait EventIDValidator {
    fn validate_event_id(&self, event: &Event) -> Result<(), EventIDValidatorError>;
}

pub struct Sha256EventIDValidator;

impl Sha256EventIDValidator {
    fn generate_event_id(&self, event: &Event) -> String {
        let serialized = json!([
            0, 
            event.public_key, 
            event.created_at, 
            event.kind.clone() as u32, 
            event.tags, 
            event.content]).to_string();

        let mut hasher = Sha256::new();
        hasher.update(serialized.as_bytes());
        let hash = hasher.finalize();

        format!("{:x}", hash)
    }
}

impl EventIDValidator for Sha256EventIDValidator {
    fn validate_event_id(&self, event: &Event) -> Result<(), EventIDValidatorError> {
        let generated_id = self.generate_event_id(event);
        if generated_id != event.id {
            return Err(EventIDValidatorError::EventIDMismatch)
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventIDValidatorError {
    EventIDMismatch
}
