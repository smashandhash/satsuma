use crate::infrastructure::{
    nostr_event::NostrEvent,
    relay_publisher::RelayPublisher,
    relay_publisher::RelayPublisherError
};

#[derive(Default)]
pub struct RelayManager;

impl RelayPublisher for RelayManager {
    fn publish(&self, event: NostrEvent) -> Result<(), RelayPublisherError> {
        println!("Publishing event: {:?}", event);
        Ok(())
    }
}
