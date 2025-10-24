use satsuma::domain::services::public_key_validator::{
    PublicKeyValidator,
    PublicKeyValidatorError
};

pub struct PublicKeyValidatorStub {
    pub simulated_error: Option<PublicKeyValidatorError>
}

impl PublicKeyValidator for PublicKeyValidatorStub {
    fn validate_public_key(&self, _public_key: &str) -> Result<(), PublicKeyValidatorError> {
        self.simulated_error.clone().map_or(Ok(()), Err)
    }
}
