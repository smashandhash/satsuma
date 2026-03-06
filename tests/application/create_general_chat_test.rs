#[cfg(test)]
mod tests {
    use satsuma::{
        domain::chat_container::{
            ChatContainer,
            ChatContainerContext,
            ChatContainerGroupType,
        },
        infrastructure::chat_container_repository::ChatContainerRepository,
    };
    use crate::helper::chat_container_repository_stub::ChatContainerRepositoryStub;
    use std::sync::Arc;

    pub trait CreateGroupChatUseCase {
        fn execute(&self, creator_public_key: &str, admins_public_key: Vec<&str>, member_public_keys: Vec<&str>) -> ChatContainer;
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
    }

    impl<R: ChatContainerRepository> CreateGroupChatUseCase for CreateGroupChatUseCaseImplementation {
        fn execute(&self, creator_public_key: &str, admins_public_key: Vec<&str>, member_public_keys: Vec<&str>) -> ChatContainer {
        }
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
