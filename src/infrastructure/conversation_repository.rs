use crate::domain::conversation::Conversation;

pub trait ConversationRepository {
    fn save(&mut self, conversation: Conversation);
}
