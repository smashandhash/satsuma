#[cfg(test)]
mod tests {
    use satsuma::domain::services::public_key_validator::{
        PublicKeyValidator,
        DefaultPublicKeyValidator,
        PublicKeyValidatorError
    };

    #[test]
    fn public_key_has_invalid_length() {
        let public_key = "npub100";
        let sut = DefaultPublicKeyValidator;
        let result = sut.validate_public_key(public_key);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PublicKeyValidatorError::InvalidPublicKeyLength);
    }

    #[test]
    fn public_key_not_hex_encoded() {
        let public_key = "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz";
        let sut = DefaultPublicKeyValidator;
        let result = sut.validate_public_key(public_key);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PublicKeyValidatorError::PublicKeyNotHexEncoded);
    }
}
