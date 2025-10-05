use crate::infrastructure::nostr_event::NostrEvent;

#[derive(Default)]
pub struct RelayManager;

impl RelayManager {
    pub fn publish(&self, event: NostrEvent) -> Result<(), String> {
        println!("Publishing event: {:?}", event);
        Ok(())
    }
}
