use crate::domain::message::Message;

pub trait MessageRepository {
    fn find_conversation(&self, sender_public_key: String, recipient_public_key: String) -> Vec<Message>;
}
