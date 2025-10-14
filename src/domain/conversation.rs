use chrono::{DateTime, Utc};

use super::message::Message;

#[derive(Debug, Clone, PartialEq)]
pub struct Conversation {
    pub id: u64,
    pub creator_public_key: String,
    pub participant_public_keys: Vec<String>,
    pub messages: Vec<Message>,
    pub created_at: DateTime<Utc>,
}

impl Conversation {
    pub fn new(id: u64, creator_public_key: &str, participant_public_keys: Vec<&str>) -> Self {
        Self {
            id,
            creator_public_key: creator_public_key.to_string(),
            participant_public_keys: participant_public_keys.into_iter().map(|key| key.to_string()).collect(),
            messages: Vec::new(),
            created_at: Utc::now(),
        }
    }

    pub fn add_message(&mut self, message: Message) -> bool {
        let valid_sender = self.participant_public_keys.contains(&message.sender_public_key);
        
        if valid_sender {
            self.messages.push(message);
            true
        } else {
            false
        }
    }

    pub fn add_participant(&mut self, user_public_key: String) -> Result<(), String> {
        if self.participant_public_keys.contains(&user_public_key) {
            return Err("A `Conversation` cannot contain duplicate participants.".to_string());
        }
        self.participant_public_keys.push(user_public_key);
        Ok(())
    }

    pub fn remove_participant(&mut self, user_public_key: String) {
        self.participant_public_keys.retain(|target_public_key| *target_public_key != user_public_key);
    }
}
