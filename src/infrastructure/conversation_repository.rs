use crate::domain::conversation::Conversation;

pub trait ConversationRepository {
    fn save(&mut self, conversation: Conversation);
    fn load(&mut self, user_public_key: String) -> Vec<Conversation>;
}
