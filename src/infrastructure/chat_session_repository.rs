use crate::domain::chat_session::ChatSession;

pub trait ChatSessionRepository {
    fn save(&self, chat_session: ChatSession) -> Result<(), ChatSessionRepositoryError>;
    fn load_by_container_id(&self, chat_container_id: String) -> Result<Vec<ChatSession>, ChatSessionRepositoryError>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum ChatSessionRepositoryError {
    SaveFailed,
    NoChatSessionFound
}
