use crate::domain::chat_session::ChatSession;

pub trait ChatSessionRepository {
    fn save(&self, chat_session: ChatSession);
    fn load(&mut self, user_public_key: String) -> Vec<ChatSession>;
}
