#[cfg(test)]
mod tests {
    use satsuma::domain::services::public_key_validator::{
        PublicKeyValidator,
        DefaultPublicKeyValidator,
        PublicKeyValidatorError
    };
    use rstest::rstest;

    #[rstest]
    #[case("Proper public key", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", None)]
    #[case("Invalid length", "npub100", Some(PublicKeyValidatorError::InvalidPublicKeyLength))]
    #[case("Not hex-encoded", "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz", Some(PublicKeyValidatorError::PublicKeyNotHexEncoded))]
    fn public_key_validator(
        #[case] _label: &str,
        #[case] public_key: &str,
        #[case] expected_error: Option<PublicKeyValidatorError>
        ) {
        let sut = DefaultPublicKeyValidator;
        let result = sut.validate_public_key(public_key);

        if let Some(error) = expected_error {
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), error);
        } else {
            assert!(result.is_ok());
        }
    }
}
