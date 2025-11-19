use crate::domain::chat_session::ChatSession;

pub trait ChatSessionRepository {
    fn save(&self, chat_session: ChatSession);
    fn load(&mut self, chat_session_id: String) -> Vec<ChatSession>;
}
