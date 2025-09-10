use crate::domain::user::User;
use crate::domain::message::Message;

pub struct SendMessageUseCase;

impl SendMessageUseCase {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, sender: &User, recipient: &User, content: &str) -> Result<Message, String> {
        if content.is_empty() {
            return Err("Sender cannot send empty message".to_string());
        }
        let message = Message::new(1, sender.id, recipient.id, content);
        Ok(message)
    }
}
