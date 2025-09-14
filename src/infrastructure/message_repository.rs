use crate::domain::message::Message;

pub trait MessageRepository {
    fn find_conversation(&self, sender_id: u64, recipient_id: u64) -> Vec<Message>;
}
