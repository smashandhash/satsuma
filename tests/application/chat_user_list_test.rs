#[cfg(test)]
mod tests {
    use satsuma::{
        domain::{
            user::User,
            chat_container::{
                ChatContainer,
                ChatContainerContext,
            },
        },
        infrastructure::{
            chat_container_repository::{
                ChatContainerRepository,
                ChatContainerRepositoryError,
            },
            user_repository::{
                UserRepository,
                UserRepositoryError,
            },
        },
    };
    use crate::helper::{
        chat_container_repository_stub::ChatContainerRepositoryStub,
        user_repository_stub::UserRepositoryStub,
    };

    pub trait GetChatUserListUseCase {
        fn execute(&self, chat_container_id: String) -> Result<Vec<User>, GetChatUserListUseCaseError>;
    }

    pub struct GetChatUserListUseCaseImplementation<CCR: ChatContainerRepository, UR: UserRepository> {
        chat_container_repository: CCR,
        user_repository: UR
    }

    impl<CCR: ChatContainerRepository, UR: UserRepository> GetChatUserListUseCaseImplementation<CCR, UR> {
        pub fn new(chat_container_repository: CCR, user_repository: UR) -> Self {
            Self {
                chat_container_repository,
                user_repository
            }
        }
    }

    impl<CCR: ChatContainerRepository, UR: UserRepository> GetChatUserListUseCase for GetChatUserListUseCaseImplementation<CCR, UR> {
        fn execute(&self, chat_container_id: String) -> Result<Vec<User>, GetChatUserListUseCaseError> {
            let chat_container = self.chat_container_repository.load(chat_container_id).map_err(|e| GetChatUserListUseCaseError::ChatContainerRepositoryError(e))?;

            let mut result: Vec<User> = Vec::new();
            for public_key in chat_container.participant_public_keys.iter() {
                result.push(self.user_repository.load(public_key.to_string()).map_err(|e| GetChatUserListUseCaseError::UserRepositoryError(e))?);
            }

            Ok(result)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum GetChatUserListUseCaseError {
        ChatContainerRepositoryError(ChatContainerRepositoryError),
        UserRepositoryError(UserRepositoryError),
    }

    #[test]
    fn get_chat_user_list() {
        let user_public_key = "user_public_key".to_string();
        let other_public_key = "other_public_key".to_string();
        let chat_container = ChatContainer::new(
            "id".to_string(), 
            ChatContainerContext::Direct {
                other_public_key: other_public_key.clone()
            }, 
            vec![user_public_key.clone(), other_public_key.clone()]
        );
        let chat_container_repository = ChatContainerRepositoryStub::new(None, Some(chat_container));
        let user_repository = UserRepositoryStub::new(None);
        let sut = GetChatUserListUseCaseImplementation::new(chat_container_repository, user_repository);
        let result = sut.execute("chat_container_id".to_string());

        assert!(result.is_ok());
    }

    #[test]
    fn chat_container_repository_error_failed_to_get_chat_user_list() {
        let chat_container_repository_simulated_error = ChatContainerRepositoryError::ContainerNotFound;
        let chat_container_repository = ChatContainerRepositoryStub::new(Some(chat_container_repository_simulated_error.clone()), None);
        let user_repository = UserRepositoryStub::new(None);
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
        let chat_container_repository = ChatContainerRepositoryStub::new(None, Some(chat_container));
        let user_repository = UserRepositoryStub::new(Some(user_repository_simulated_error.clone()));
        let sut = GetChatUserListUseCaseImplementation::new(chat_container_repository, user_repository);
        let result = sut.execute("chat_container_id".to_string());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), GetChatUserListUseCaseError::UserRepositoryError(user_repository_simulated_error.clone()));
    }
}
