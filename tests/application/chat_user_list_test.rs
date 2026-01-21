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
            profile_repository::{
                ProfileRepository,
                ProfileRepositoryError,
            },
        },
    };
    use crate::helper::{
        chat_container_repository_stub::ChatContainerRepositoryStub,
        profile_repository_stub::ProfileRepositoryStub,
    };

    pub trait GetChatUserListUseCase {
        fn execute(&self, chat_container_id: String) -> Result<Vec<User>, GetChatUserListUseCaseError>;
    }

    pub struct GetChatUserListUseCaseImplementation<CCR: ChatContainerRepository, PR: ProfileRepository> {
        chat_container_repository: CCR,
        profile_repository: PR
    }

    impl<CCR: ChatContainerRepository, PR: ProfileRepository> GetChatUserListUseCaseImplementation<CCR, PR> {
        pub fn new(chat_container_repository: CCR, profile_repository: PR) -> Self {
            Self {
                chat_container_repository,
                profile_repository
            }
        }
    }

    impl<CCR: ChatContainerRepository, PR: ProfileRepository> GetChatUserListUseCase for GetChatUserListUseCaseImplementation<CCR, PR> {
        fn execute(&self, chat_container_id: String) -> Result<Vec<User>, GetChatUserListUseCaseError> {
            let chat_container = self.chat_container_repository.load(chat_container_id).map_err(|e| GetChatUserListUseCaseError::ChatContainerRepositoryError(e))?;

            let mut result: Vec<User> = Vec::new();
            for public_key in chat_container.participant_public_keys.iter() {
                result.push(self.profile_repository.load(public_key.to_string()).map_err(|e| GetChatUserListUseCaseError::ProfileRepositoryError(e))?);
            }

            Ok(result)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum GetChatUserListUseCaseError {
        ChatContainerRepositoryError(ChatContainerRepositoryError),
        ProfileRepositoryError(ProfileRepositoryError),
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
        let profile_repository = ProfileRepositoryStub::new(None);
        let sut = GetChatUserListUseCaseImplementation::new(chat_container_repository, profile_repository);
        let result = sut.execute("chat_container_id".to_string());

        assert!(result.is_ok())
    }
}
