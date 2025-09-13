use crate::domain::message::Message;

pub trait MessageRepository {
    fn find_conversation(&self, sender_id: &str, recipient_id: &str) -> Vec<Message>;
}
