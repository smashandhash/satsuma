pub struct Event {
    pub id: String,
    pub public_key: String,
    pub created_at: u64,
    pub kind: u32,
    pub tags: Vec<Vec<String>>,
    pub content: String
}
