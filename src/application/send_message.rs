use crate::domain::user::User;
use crate::domain::message::Message;

pub struct SendMessageUseCase;

impl SendMessageUseCase {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, sender: &User, recipient: &User, content: &str) -> Message {
        let message = Message::new(1, sender.id, recipient.id, content);
        return message;
    }
}
