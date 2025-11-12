use satsuma::infrastructure::relay_publisher::{
    RelayPublisher,
    RelayPublisherError
};
use nostr_sdk::Metadata;

pub struct RelayPublisherStub {
    pub simulated_error: Option<RelayPublisherError>
}

impl RelayPublisher for RelayPublisherStub {
    fn publish(&self, _metadata: &Metadata) -> Result<(), RelayPublisherError> {
        self.simulated_error.clone().map_or(Ok(()), Err)
    }
}
