#[derive(Debug, Clone, PartialEq)]
#[repr(u32)]
pub enum EventKind {
    Metadata = 0,
    DirectMessage = 17,
    ChannelOrGroupMessage = 28,
}
