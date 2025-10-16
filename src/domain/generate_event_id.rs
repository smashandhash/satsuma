use sha2::{Sha256, Digest};
use serde_json::json;

pub fn generate_event_id(
    public_key: &str,
    created_at: u64,
    kind: u32,
    tags: &Vec<Vec<String>>,
    content: &str) -> String {
    let serialized = json!([0, public_key, created_at, kind, tags, content]).to_string();
    let hash = Sha256::digest(serialized.as_bytes());
    format!("{:x}", hash)
}
