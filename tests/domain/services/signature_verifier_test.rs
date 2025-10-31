#[cfg(test)]
mod tests {
    use satsuma::domain::services::signature_verifier::{
        SignatureVerifier,
        Secp256k1SignatureVerifier,
        SignatureVerifierError
    };
    use rstest::rstest;
    
    #[rstest]
    #[case("Invalid ID Hex", Some("id".to_string()), None, None, SignatureVerifierError::InvalidIDHex)]
    #[case("Invalid Public Key Hex", None, Some("public_key".to_string()), None, SignatureVerifierError::InvalidPublicKeyHex)]
    fn signature_verifier_errors(
        #[case] _label: &str,
        #[case] mocked_id: Option<String>,
        #[case] mocked_public_key: Option<String>,
        #[case] mocked_signature: Option<String>,
        #[case] expected_error: SignatureVerifierError
        ) {
        let mut id = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string();
        let mut public_key = "public_key".to_string();
        let mut signature = "signature".to_string();

        if let Some(mocked_id) = mocked_id {
            id = mocked_id;
        }
        if let Some(mocked_public_key) = mocked_public_key {
            public_key = mocked_public_key;
        }
        if let Some(mocked_signature) = mocked_signature {
            signature = mocked_signature;
        }

        let sut = Secp256k1SignatureVerifier;
        let result = sut.verify_signature(&id, &public_key, &signature);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), expected_error);
    }
}
