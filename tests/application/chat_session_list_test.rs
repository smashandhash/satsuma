#[cfg(test)]
mod tests {
    use satsuma::{
        application::chat_session_list::{
            ChatSessionListUseCase,
            ChatSessionListUseCaseImplementation,
            ChatSessionListUseCaseError,
        },
        domain::chat_session::{
            ChatSession,
            ChatSessionContext,
        },
        infrastructure::chat_session_repository::ChatSessionRepositoryError,
    };
    use crate::helper::chat_session_repository_stub::ChatSessionRepositoryStub;
    use std::sync::Arc;

    #[test]
    fn successfully_load_chat_session_list() {
        let chat_sessions = vec![ChatSession::new("id".to_string(), "container_id".to_string(), ChatSessionContext::Root)];
        let repository = Arc::new(ChatSessionRepositoryStub::new(
            Some(chat_sessions.clone()),
            None,
        ));
        let sut = ChatSessionListUseCaseImplementation::new(repository);

        let result = sut.execute("id".to_string());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), chat_sessions.clone());
    }

    #[test]
    fn repository_failed_fail_to_load() {
        let simulated_error = ChatSessionRepositoryError::NoChatSessionFound;
        let repository = Arc::new(ChatSessionRepositoryStub::new(
            None,
            Some(simulated_error.clone())
        ));
        let sut = ChatSessionListUseCaseImplementation::new(repository);

        let result = sut.execute("id".to_string());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ChatSessionListUseCaseError::RepositoryError(simulated_error.clone()));
    }
}
