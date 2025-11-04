use satsuma::domain::{
    event::Event,
    services::nostr_event_validator::{NostrEventValidator, NostrEventValidatorError},
};

pub struct NostrEventValidatorStub {
    pub simulated_error: Option<NostrEventValidatorError>,
}

impl NostrEventValidator for NostrEventValidatorStub {
    fn validate(&self, _event: &Event) -> Result<(), NostrEventValidatorError> {
        self.simulated_error.clone().map_or(Ok(()), Err)
    }
}
