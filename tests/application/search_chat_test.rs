#[cfg(test)]
mod tests {
    use satsuma::{
        application::search_chat::{
            SearchChatUseCase,
            SearchChatUseCaseImplementation,
            SearchChatUseCaseError,
        },
        infrastructure::chat_container_repository::ChatContainerRepositoryError,
    };
    use crate::helper::chat_container_repository_stub::ChatContainerRepositoryStub;
    use std::sync::Arc;

    #[test]
    fn successfully_search_chat() {
        let repository = Arc::new(ChatContainerRepositoryStub { simulated_error: None, mocked_chat_container: None });
        let sut = SearchChatUseCaseImplementation::new(repository);

        let result = sut.execute("keyword".to_string());

        assert!(result.is_ok());
    }

    #[test]
    fn repository_error_failed_to_search_chat() {
        let error = ChatContainerRepositoryError::ContainerNotFound;
        let repository = Arc::new(ChatContainerRepositoryStub { simulated_error: Some(error.clone()), mocked_chat_container: None });
        let sut = SearchChatUseCaseImplementation::new(repository);

        let result = sut.execute("keyword".to_string());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SearchChatUseCaseError::RepositoryError(error.clone()));
    }
}
