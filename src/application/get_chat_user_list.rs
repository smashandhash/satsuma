use crate::{
    domain::user::User,
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
use std::sync::Arc;

pub trait GetChatUserListUseCase {
    fn execute(&self, chat_container_id: String) -> Result<Vec<User>, GetChatUserListUseCaseError>;
}

pub struct GetChatUserListUseCaseImplementation<CCR: ChatContainerRepository, UR: UserRepository> {
    chat_container_repository: Arc<CCR>,
    user_repository: Arc<UR>
}

impl<CCR: ChatContainerRepository, UR: UserRepository> GetChatUserListUseCaseImplementation<CCR, UR> {
    pub fn new(chat_container_repository: Arc<CCR>, user_repository: Arc<UR>) -> Self {
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

