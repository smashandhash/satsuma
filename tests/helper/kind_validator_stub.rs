use satsuma::domain::services::kind_validator::{
    KindValidator,
    KindValidatorError
};

pub struct KindValidatorStub {
    pub simulated_error: Option<KindValidatorError>
}

impl KindValidator for KindValidatorStub {
    fn validate_kind(&self, _kind: u32) -> Result<(), KindValidatorError> {
        self.simulated_error.clone().map_or(Ok(()), Err)
    }
}
