use satsuma::domain::{
    event::Event,
    services::event_id_validator::{
        EventIDValidator,
        EventIDValidatorError
    },
};

pub struct EventIDValidatorStub {
    pub simulated_error: Option<EventIDValidatorError>
}

impl EventIDValidator for EventIDValidatorStub {
    fn validate_event_id(&self, _event: &Event) -> Result<(), EventIDValidatorError> {
        self.simulated_error.clone().map_or(Ok(()), Err)
    }
}
