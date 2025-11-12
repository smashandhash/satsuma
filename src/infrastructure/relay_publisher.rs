use nostr_sdk::Metadata;

pub trait RelayPublisher {
    fn publish(&self, metadata: &Metadata) -> Result<(), RelayPublisherError>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum RelayPublisherError {
    ConnectionFailed,
    PublishTimeout,
    RejectedRelay,
    SerializationFailed,
    Unknown(String)
}
