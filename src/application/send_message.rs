use crate::domain::user::User;
use crate::domain::message::Message;

pub struct SendMessageUseCase {
    max_length: usize,
}

impl SendMessageUseCase {
    pub fn new(max_length: usize) -> Self {
        Self { max_length }
    }

    pub fn execute(&self, sender: &User, recipient: &User, content: &str) -> Result<Message, String> {
        let trimmed_content = content.trim();
        if trimmed_content.is_empty() {
            return Err("Sender cannot send empty message".to_string());
        }

        if trimmed_content.chars().count() > self.max_length {
            return Err("Message too long".to_string());
        }

        let message = Message::new(1, sender.id, recipient.id, trimmed_content);
        Ok(message)
    }
}
