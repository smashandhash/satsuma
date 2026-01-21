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
        infrastructure::chat_container_repository::{
            ChatContainerRepository,
            ChatContainerRepositoryError,
        },
    };
    use crate::helper::chat_container_repository_stub::ChatContainerRepositoryStub;

    pub trait GetChatUserListUseCase {
        fn execute(&self, chat_container_id: String) -> Vec<User>;
    }

    pub struct GetChatUserListUseCaseImplementation<R: ChatContainerRepository> {
        repository: R,
    }

    impl<R: ChatContainerRepository> GetChatUserListUseCaseImplementation<R> {
        pub fn new(repository: R) -> Self {
            Self {
                repository: repository
            }
        }
    }

    impl<R: ChatContainerRepository> GetChatUserListUseCase for GetChatUserListUseCaseImplementation<R> {
        fn execute(&self, chat_container_id: String) -> Option<Vec<User>> {
            return Vec::new()
        }
    }

    #[test]
    fn get_chat_user_list() {
        let user_public_key = "user_public_key".to_string();
        let other_public_key = "other_public_key".to_string();
        let chat_container = ChatContainer::new(
            "id".to_string(), 
            ChatContainerContext::Direct(other_public_key.clone()), 
            [user_public_key.clone(), other_public_key.clone()]
        );
        let repository = ChatContainerRepositoryStub::new(None, Some(chat_container));
        let sut = GetChatUserListUseCaseImplementation::new(repository);
        let result = sut.execute("chat_container_id".to_string());

        assert!(!result.is_empty())
    }
}
