use chrono::Utc;
use sha2::{Sha256, Digest};

#[derive(Debug, Clone)]
pub struct NostrEvent {
    pub id: String,
    pub pubkey: String,
    pub created_at: i64,
    pub kind: u32,
    pub content: String,
}

impl NostrEvent {
    pub fn new(kind: u32, content: String, pubkey: &str) -> Self {
        let created_at = Utc::now().timestamp();
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}", pubkey, created_at, content));
        let id = format!("{:x}", hasher.finalize());

        Self {
            id,
            pubkey: pubkey.to_string(),
            created_at,
            kind,
            content,
        }
    }
}
