use secp256k1::{ Secp256k1, XOnlyPublicKey, schnorr::Signature as SchnorrSignature };
use hex::FromHex;

pub fn verify_signature(id: &str, public_key: &str, signature: &str) -> Result<(), VerifySignatureError> {
    let secp = Secp256k1::verification_only();

    let id_bytes = <[u8;32]>::from_hex(id).map_err(|_| VerifySignatureError::InvalidIDHex)?;
    let public_key_bytes = <[u8;32]>::from_hex(public_key).map_err(|_| VerifySignatureError::InvalidPublicKeyHex)?;
    let signature_bytes = <[u8;64]>::from_hex(signature).map_err(|_| VerifySignatureError::InvalidSignatureHex)?;

    let x_only_public_key = XOnlyPublicKey::from_byte_array(public_key_bytes).map_err(|_| VerifySignatureError::InvalidPublicKeyFormat)?;
    let signature = SchnorrSignature::from_byte_array(signature_bytes);

    Ok(secp.verify_schnorr(&signature, &id_bytes, &x_only_public_key).map_err(|_| VerifySignatureError::ForgedOrMalformedSignature)?)
}

#[derive(Debug, PartialEq)]
pub enum VerifySignatureError {
    InvalidIDHex,
    InvalidPublicKeyHex,
    InvalidSignatureHex,
    InvalidPublicKeyFormat,
    ForgedOrMalformedSignature
}
