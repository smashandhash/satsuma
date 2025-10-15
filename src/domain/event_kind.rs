#[derive(Debug, Clone, PartialEq)]
#[repr(u32)]
pub enum EventKind {
    Metadata = 0,
    PrivateOrGroupMessage = 14,
    PublicMessage = 42,
}
