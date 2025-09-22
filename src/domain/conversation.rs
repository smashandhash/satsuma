use chrono::{DateTime, Utc};

use super::message::Message;

#[derive(Debug, Clone)]
pub struct Conversation {
    pub id: u64,
    pub participant_ids: Vec<u64>,
    pub messages: Vec<Message>,
    pub created_at: DateTime<Utc>,
}

impl Conversation {
    pub fn new(id: u64, participant_ids: Vec<u64>) -> Self {
        Self {
            id,
            participant_ids,
            messages: Vec::new(),
            created_at: Utc::now(),
        }
    }

    pub fn add_message(&mut self, message: Message) -> bool {
        let valid_sender = self.participant_ids.contains(&message.sender_id);
        let valid_recipient = self.participant_ids.contains(&message.recipient_id);
        
        if valid_sender && valid_recipient {
            self.messages.push(message);
            true
        } else {
            false
        }
    }

    pub fn add_participant(&mut self, user_id: u64) -> Result<(), String> {
        if self.participant_ids.contains(&user_id) {
            return Err("A `Conversation` cannot contain duplicate participants.".to_string());
        }
        self.participant_ids.push(user_id);
        Ok(())
    }
}
