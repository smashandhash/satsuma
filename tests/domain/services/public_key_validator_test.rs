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
}
