use async_trait::async_trait;
use nostr_sdk::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum KeyProviderError {
    InvalidKey(String),
}

#[async_trait]
pub trait KeyProvider {
    async fn parse_secret_key(&self, key: &str) -> Result<Keys, KeyProviderError>;
}

pub struct NostrKeyProvider;

#[async_trait]
impl KeyProvider for NostrKeyProvider {
    async fn parse_secret_key(&self, key: &str) -> Result<Keys, KeyProviderError> {
        Keys::parse(key)
            .map_err(|e| KeyProviderError::InvalidKey(e.to_string()))
    }
}

