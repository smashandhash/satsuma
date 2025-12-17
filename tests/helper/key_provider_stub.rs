use satsuma::infrastructure::key_provider::{
    KeyProvider,
    KeyProviderError
};
use async_trait::async_trait;
use nostr_sdk::prelude::*;

pub struct KeyProviderStub {
    pub simulated_error: Option<KeyProviderError>,
}

impl KeyProviderStub {
    pub fn new(simulated_error: Option<KeyProviderError>) -> Self {
        Self { simulated_error }
    }
}

#[async_trait]
impl KeyProvider for KeyProviderStub {
    async fn parse_secret_key(&self, _key: &str) -> Result<Keys, KeyProviderError> {
        self.simulated_error.clone().map_or(Ok(Keys::generate()), Err)
    }
}
