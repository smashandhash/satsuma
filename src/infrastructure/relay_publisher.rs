use crate::infrastructure::nostr_event::NostrEvent;

pub trait RelayPublisher {
    fn publish(&self, event: NostrEvent) -> Result<(), RelayPublisherError>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum RelayPublisherError {
    ConnectionFailed,
    PublishTimeout,
    RejectedRelay,
    SerializationFailed,
    Unknown(String)
}
