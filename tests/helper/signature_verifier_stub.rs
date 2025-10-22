use satsuma::domain::services::signature_verifier::{
    SignatureVerifier,
    SignatureVerifierError
};

pub struct SignatureVerifierStub {
    pub simulated_error: Option<SignatureVerifierError>
}

impl SignatureVerifier for SignatureVerifierStub {
    fn verify_signature(&self,
        _id: &str,
        _public_key: &str,
        _signature: &str,
    ) -> Result<(), SignatureVerifierError> {
        self.simulated_error.clone().map_or(Ok(()), Err)
    }
}
