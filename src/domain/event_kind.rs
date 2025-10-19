#[derive(Debug, Clone, PartialEq)]
#[repr(u32)]
pub enum EventKind {
    Metadata = 0,
    PrivateOrGroupMessage = 14,
    PublicMessage = 42,
}

impl EventKind {
    pub fn get_event_kind(value: u32) -> Result<Self, String> {
        match value {
            0 => Ok(EventKind::Metadata),
            14 => Ok(EventKind::PrivateOrGroupMessage),
            42 => Ok(EventKind::PublicMessage),
            _ => Err(format!("Invalid kind value: {}", value))
        }
    }
}
