use satsuma::{
    domain::chat_container::ChatContainer,
    infrastructure::chat_container_repository::{
        ChatContainerRepository,
        ChatContainerRepositoryError
    }
};

pub struct ChatContainerRepositoryStub {
    pub simulated_error: Option<ChatContainerRepositoryError>,
    pub mocked_chat_container: Option<ChatContainer>,
}

impl ChatContainerRepository for ChatContainerRepositoryStub {
    fn save(&self, _container: ChatContainer) -> Result<(), ChatContainerRepositoryError> {
        self.simulated_error.clone().map_or(Ok(()), Err)
    }

    fn load(&self, _container_id: String) -> Result<ChatContainer, ChatContainerRepositoryError> {
        if let Some(err) = &self.simulated_error {
            Err(err.clone())
        } else {
            Ok(self.mocked_chat_container.clone().expect("mocked_container must be provided when no error"))
        }
    }

    fn search(&self, _keyword: String) -> Result<Vec<ChatContainer>, ChatContainerRepositoryError> {
        if let Some(err) = &self.simulated_error {
            Err(err.clone())
        } else {
            Ok(Vec::new())
        }
    }
}
