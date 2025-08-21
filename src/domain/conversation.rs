pub struct Conversation {
    pub id: u64,
    pub participants: Vec<u64>,
    pub messages: Vec<Message>,
}
