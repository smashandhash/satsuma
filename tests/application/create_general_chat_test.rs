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
    use md5::compute;
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

        fn generate_chat_session_id(&self, member_public_keys: Vec<&str>) -> String {
            let mut keys: Vec<String> = member_public_keys.into_iter().map(|v| v.to_string()).collect();
            keys.sort();

            let joined_keys = format!("{}:{}", keys[0], keys[1]);
            format!("{:x}", compute(joined_keys))
        }
    }

    impl<R: ChatContainerRepository> CreateGroupChatUseCase for CreateGroupChatUseCaseImplementation<R> {
        fn execute(&self, creator_public_key: &str, admins_public_key: Vec<&str>, member_public_keys: Vec<&str>) -> Result<ChatContainer, CreateGroupChatUseCaseError> {
            let chat_container = ChatContainer::new(
                self.generate_chat_session_id(member_public_keys.clone()),
                ChatContainerContext::Group {
                    group_type: ChatContainerGroupType::Channel,
                    creator_public_key: creator_public_key.to_string(),
                    admins_public_key: admins_public_key.into_iter().map(|v| v.to_string()).collect(),
                },
                member_public_keys.into_iter().map(|v| v.to_string()).collect()
            );
            self.repository
                .save(chat_container.clone())
                .map_err(|e| CreateGroupChatUseCaseError::RepositoryError(e))?;

            Ok(chat_container)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum CreateGroupChatUseCaseError {
        RepositoryError(ChatContainerRepositoryError)
    }

    #[test]
    fn successfully_create_a_general_chat() {
        let creator_public_key = "sender_public_key";
        let admins_public_key = vec![creator_public_key];
        let member_public_keys = vec![creator_public_key, "first_member_key", "second_member_key"];
        let chat_container = ChatContainer::new(
            "id".to_string(),
            ChatContainerContext::Group {
                group_type: ChatContainerGroupType::Channel,
                creator_public_key: creator_public_key.to_string(),
                admins_public_key: admins_public_key.clone().into_iter().map(|k| k.to_string()).collect()
            },
            member_public_keys.clone().into_iter().map(|k| k.to_string()).collect(),
        );
        let repository = Arc::new(ChatContainerRepositoryStub::new(
                None,
                Some(chat_container.clone())
        ));
        let sut = CreateGroupChatUseCaseImplementation::new(repository);

        let result = sut.execute(creator_public_key, admins_public_key, member_public_keys);
        let given_chat_container = result.unwrap();

        assert_eq!(given_chat_container.context, chat_container.context);
        assert_eq!(given_chat_container.participant_public_keys, chat_container.participant_public_keys);
    }
}
