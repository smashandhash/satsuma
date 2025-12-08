use satsuma::{
    domain::chat_session::ChatSession,
    infrastructure::chat_session_repository::{
        ChatSessionRepository,
        ChatSessionRepositoryError
    }
};

pub struct ChatSessionRepositoryStub {
    pub mocked_chat_sessions: Option<Vec<ChatSession>>,
    pub simulated_error: Option<ChatSessionRepositoryError>,
}

impl ChatSessionRepository for ChatSessionRepositoryStub {
    fn save(&self, _chat_session: ChatSession) -> Result<(), ChatSessionRepositoryError> {
        self.simulated_error.clone().map_or(Ok(()), Err)
    }

    fn load_by_container_id(&self, _chat_container_id: String) -> Result<Vec<ChatSession>, ChatSessionRepositoryError> {
        if let Some(chat_sessions) = self.mocked_chat_sessions.clone() {
            Ok(chat_sessions)
        } else {
            Err(self.simulated_error.clone().expect("Should mock either one of them"))
        }
    }
}
