#[cfg(test)]
mod tests {
    use satsuma::{
        domain::chat_container::{
            ChatContainer,
            ChatContainerContext,
            ChatContainerGroupType,
        },
        infrastructure::chat_container_repository::{
            ChatContainerRepository,
            ChatContainerRepositoryError,
        },
    };
    use crate::helper::chat_container_repository_stub::ChatContainerRepositoryStub;
    use std::sync::Arc;

    pub trait CreateGroupChatUseCase {
        fn execute(&self, creator_public_key: &str, admins_public_key: Vec<&str>, member_public_keys: Vec<&str>) -> Result<ChatContainer, CreateGroupChatUseCaseError>;
    }

    pub struct CreateGroupChatUseCaseImplementation<R: ChatContainerRepository> {
        repository: Arc<R>
    }

    impl<R: ChatContainerRepository> CreateGroupChatUseCaseImplementation<R> {
        pub fn new(repository: Arc<R>) -> Self {
            Self {
                repository
            }
        }

        fn generate_chat_session_id(&self, sender_public_key: &str, recipient_public_key: &str) -> String {
            let mut keys = vec![sender_public_key.to_string(), recipient_public_key.to_string()];
            keys.sort();

            let joined_keys = format!("{}:{}", keys[0], keys[1]);
            format!("{:x}", compute(joined_keys))
        }
    }

    impl<R: ChatContainerRepository> CreateGroupChatUseCase for CreateGroupChatUseCaseImplementation {
        fn execute(&self, creator_public_key: &str, admins_public_key: Vec<&str>, member_public_keys: Vec<&str>) -> Result<ChatContainer, CreateGroupChatUseCaseError> {
            let chat_container = ChatContainer::new(
                self.generate_chat_session_id(creator_public_key, member_public_keys)
            self.repository.save(
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum CreateGroupChatUseCaseError {
        RepositoryError(ChatContainerRepositoryError)
    }

    #[test]
    fn successfully_create_a_general_chat() {
        let creator_public_key = "sender_public_key".to_string();
        let admins_public_key = vec![creator_public_key.clone()];
        let member_public_keys = vec![creator_public_key.clone(), "first_member_key".to_string(), "second_member_key".to_string()];
        let chat_container = ChatContainer::new(
            "id".to_string(),
            ChatContainerContext::Group {
                group_type: ChatContainerGroupType::Channel,
                creator_public_key,
                admins_public_key
            },
            member_public_keys.clone(),
        );
        let repository = Arc::new(ChatContainerRepositoryStub::new(
                None,
                Some(chat_container.clone())
        ));
        let sut = CreateGroupChatUseCaseImplementation::new(repository);
    }
}
