use chrono::{DateTime, Utc};

use super::message::Message;

pub struct Conversation {
    pub id: u64,
    pub participant_ids: Vec<u64>,
    pub messages: Vec<Message>,
    pub created_at: DateTime<Utc>,
}
