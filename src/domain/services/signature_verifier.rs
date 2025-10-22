use secp256k1::{Secp256k1, XOnlyPublicKey, schnorr::Signature as SchnorrSignature};
use hex::FromHex;

pub trait SignatureVerifier {
    fn verify_signature(
        &self,
        id: &str,
        public_key: &str,
        signature: &str,
    ) -> Result<(), SignatureVerifierError>;
}

pub struct Secp256k1SignatureVerifier;

impl SignatureVerifier for Secp256k1SignatureVerifier {
    fn verify_signature(
        &self,
        id: &str,
        public_key: &str,
        signature: &str,
    ) -> Result<(), SignatureVerifierError> {
        let secp = Secp256k1::verification_only();

        let id_bytes = <[u8; 32]>::from_hex(id)
            .map_err(|_| SignatureVerifierError::InvalidIDHex)?;
        let public_key_bytes = <[u8; 32]>::from_hex(public_key)
            .map_err(|_| SignatureVerifierError::InvalidPublicKeyHex)?;
        let signature_bytes = <[u8; 64]>::from_hex(signature)
            .map_err(|_| SignatureVerifierError::InvalidSignatureHex)?;

        let x_only_public_key = XOnlyPublicKey::from_byte_array(public_key_bytes)
            .map_err(|_| SignatureVerifierError::InvalidPublicKeyFormat)?;
        let signature = SchnorrSignature::from_byte_array(signature_bytes);

        secp.verify_schnorr(&signature, &id_bytes, &x_only_public_key)
            .map_err(|_| SignatureVerifierError::ForgedOrMalformedSignature)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SignatureVerifierError {
    InvalidIDHex,
    InvalidPublicKeyHex,
    InvalidSignatureHex,
    InvalidPublicKeyFormat,
    ForgedOrMalformedSignature,
}
