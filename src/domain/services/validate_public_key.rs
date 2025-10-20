pub fn validate_public_key(public_key: &str) -> Result<(), ValidatePublicKeyError> {
    if public_key.len() != 64 {
        return Err(ValidatePublicKeyError::InvalidPublicKeyLength)
    }

    if hex::decode(public_key).is_err() {
        return Err(ValidatePublicKeyError::PublicKeyNotHexEncoded)
    }

    Ok(())
}

#[derive(Debug, PartialEq)]
pub enum ValidatePublicKeyError {
    InvalidPublicKeyLength,
    PublicKeyNotHexEncoded
}
