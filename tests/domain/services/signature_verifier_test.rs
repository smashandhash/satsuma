#[cfg(test)]
mod tests {
    use satsuma::domain::services::signature_verifier::{
        SignatureVerifier,
        Secp256k1SignatureVerifier,
        SignatureVerifierError
    };
    
    #[test]
    fn invalid_id_hex() {
        let id = "id";
        let public_key = "public_key";
        let signature = "signature";
        let sut = Secp256k1SignatureVerifier;
        let result = sut.verify_signature(id, public_key, signature);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SignatureVerifierError::InvalidIDHex);
    }
}
