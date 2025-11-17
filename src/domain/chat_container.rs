pub struct ChatContainer {
    pub id: String,
    pub container_type: ChatContainerType,
    pub participant_public_keys: Vec<String>
}

impl ChatContainer {
    pub fn new(id: String, container_type: ChatContainerType, participant_public_keys: Vec<String>) -> Self {
        Self {
            id,
            container_type,
            participant_public_keys
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChatContainerType {
    Direct,
    PrivateGroup,
    Channel
}
