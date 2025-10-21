use sha2::{Sha256, Digest};
use serde_json::json;
use crate::domain::event::Event;

fn generate_event_id(event: &Event) -> String {
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

pub fn validate_event_id(event: &Event) -> Result<(), ValidateEventIDError> {
    let generated_id = generate_event_id(event);
    if generated_id != event.id {
        return Err(ValidateEventIDError::EventIDMismatch)
    }

    Ok(())
}

#[derive(Debug, PartialEq)]
pub enum ValidateEventIDError {
    EventIDMismatch
}
