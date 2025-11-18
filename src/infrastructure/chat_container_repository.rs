use crate::domain::chat_container::ChatContainer;

pub trait ChatContainerRepository {
    fn save(&self, container: ChatContainer) -> Result<(), ChatContainerRepositoryError>;
    fn load(&self, container_id: String) -> Result<ChatContainer, ChatContainerRepositoryError>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChatContainerRepositoryError {
    ContainerNotFound,
    SaveFailed
}
