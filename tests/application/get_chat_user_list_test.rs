#[cfg(test)]
mod tests {
    use satsuma::{
        application::get_chat_user_list::{
            GetChatUserListUseCase,
            GetChatUserListUseCaseImplementation,
            GetChatUserListUseCaseError,
        },
        domain::chat_container::{
            ChatContainer,
            ChatContainerContext,
        },
        infrastructure::{
            chat_container_repository::ChatContainerRepositoryError,
            user_repository::UserRepositoryError,
        },
    };
    use crate::helper::{
        chat_container_repository_stub::ChatContainerRepositoryStub,
        user_repository_stub::UserRepositoryStub,
    };
    use std::sync::Arc;

    #[test]
    fn success_to_get_chat_user_list() {
        let user_public_key = "user_public_key".to_string();
        let other_public_key = "other_public_key".to_string();
        let chat_container = ChatContainer::new(
            "id".to_string(), 
            ChatContainerContext::Direct {
                other_public_key: other_public_key.clone()
            }, 
            vec![user_public_key.clone(), other_public_key.clone()]
        );
        let chat_container_repository = Arc::new(ChatContainerRepositoryStub::new(None, Some(chat_container)));
        let user_repository = Arc::new(UserRepositoryStub::new(None));
        let sut = GetChatUserListUseCaseImplementation::new(chat_container_repository, user_repository);
        let result = sut.execute("chat_container_id".to_string());

        assert!(result.is_ok());
    }

    #[test]
    fn chat_container_repository_error_failed_to_get_chat_user_list() {
        let chat_container_repository_simulated_error = ChatContainerRepositoryError::ContainerNotFound;
        let chat_container_repository = Arc::new(ChatContainerRepositoryStub::new(Some(chat_container_repository_simulated_error.clone()), None));
        let user_repository = Arc::new(UserRepositoryStub::new(None));
        let sut = GetChatUserListUseCaseImplementation::new(chat_container_repository, user_repository);
        let result = sut.execute("chat_container_id".to_string());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GetChatUserListUseCaseError::ChatContainerRepositoryError(chat_container_repository_simulated_error));
    }

    #[test]
    fn user_repository_error_failed_to_get_chat_user_list() {
        let user_repository_simulated_error = UserRepositoryError::UserNotFound;
        let user_public_key = "user_public_key".to_string();
        let other_public_key = "other_public_key".to_string();
        let chat_container = ChatContainer::new(
            "id".to_string(), 
            ChatContainerContext::Direct {
                other_public_key: other_public_key.clone()
            }, 
            vec![user_public_key.clone(), other_public_key.clone()]
        );
        let chat_container_repository = Arc::new(ChatContainerRepositoryStub::new(None, Some(chat_container)));
        let user_repository = Arc::new(UserRepositoryStub::new(Some(user_repository_simulated_error.clone())));
        let sut = GetChatUserListUseCaseImplementation::new(chat_container_repository, user_repository);
        let result = sut.execute("chat_container_id".to_string());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GetChatUserListUseCaseError::UserRepositoryError(user_repository_simulated_error.clone()));
    }
}
