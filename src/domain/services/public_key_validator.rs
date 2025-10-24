pub trait PublicKeyValidator {
    fn validate_public_key(&self, public_key: &str) -> Result<(), PublicKeyValidatorError>;
}

pub struct DefaultPublicKeyValidator;

impl PublicKeyValidator for DefaultPublicKeyValidator {
    fn validate_public_key(&self, public_key: &str) -> Result<(), PublicKeyValidatorError> {
        if public_key.len() != 64 {
            return Err(PublicKeyValidatorError::InvalidPublicKeyLength)
        }

        if hex::decode(public_key).is_err() {
            return Err(PublicKeyValidatorError::PublicKeyNotHexEncoded)
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PublicKeyValidatorError {
    InvalidPublicKeyLength,
    PublicKeyNotHexEncoded
}
