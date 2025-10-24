use satsuma::domain::services::timestamp_validator::{
    TimestampValidator,
    TimestampValidatorError
};

pub struct TimestampValidatorStub {
    pub simulated_error: Option<TimestampValidatorError>
}

impl TimestampValidator for TimestampValidatorStub {
    fn validate_timestamp(&self, _created_at: u64) -> Result<(), TimestampValidatorError> {
        self.simulated_error.clone().map_or(Ok(()), Err)
    }
}
