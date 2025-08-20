pub struct Message {
    pub id: u64,
    pub sender_id: u64,
    pub recipient_id: u64,
    pub content: String,
    pub timestamp: DateTime<Utc>
}
