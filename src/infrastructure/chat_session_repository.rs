use crate::domain::chat_session::ChatSession;

pub trait ChatSessionRepository {
    fn save(&self, chat_session: ChatSession) -> Result<(), ChatSessionRepositoryError>;
    fn load(&self, chat_session_id: String) -> Result<Vec<ChatSession>, ChatSessionRepositoryError>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum ChatSessionRepositoryError {
    SaveFailed,
    NoChatSessionFound
}
