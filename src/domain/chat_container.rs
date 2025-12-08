#[derive(Debug, Clone, PartialEq)]
pub struct ChatContainer {
    pub id: String,
    pub context: ChatContainerContext,
    pub participant_public_keys: Vec<String>,
}

impl ChatContainer {
    pub fn new(id: String, context: ChatContainerContext, participant_public_keys: Vec<String>) -> Self {
        Self {
            id,
            context,
            participant_public_keys,
        }
    }

    fn can_edit_participants(&self, actor_public_key: &str) -> bool {
        match &self.context {
            ChatContainerContext::Direct { .. } => false,
            ChatContainerContext::Group { creator_public_key, admins_public_key, .. } => {
                creator_public_key == actor_public_key || admins_public_key.contains(&actor_public_key.to_string())
            }
        }
    }

    pub fn add_participants(&mut self, actor_public_key: &str, new_participant_public_keys: Vec<String>) -> Result<(), ChatContainerError> {
        if self.context.is_direct() {
            return Err(ChatContainerError::DirectChatCannotAddParticipants)
        }

        if !self.can_edit_participants(actor_public_key) {
            return Err(ChatContainerError::PermissionDenied)
        }

        for public_key in &new_participant_public_keys {
            if self.participant_public_keys.contains(&public_key) {
                return Err(ChatContainerError::AlreadyExists)
            }
        }

        for public_key in new_participant_public_keys {
            self.participant_public_keys.push(public_key);
        }

        Ok(())
    }

    pub fn remove_participants(&mut self, actor_public_key: &str, participant_public_keys: Vec<String>) -> Result<(), ChatContainerError> {
        if self.context.is_direct() {
            return Err(ChatContainerError::DirectChatCannotRemoveParticipants)
        }

        if !self.can_edit_participants(actor_public_key) {
            return Err(ChatContainerError::PermissionDenied)
        }

        for public_key in &participant_public_keys {
            if !self.participant_public_keys.contains(&public_key) {
                return Err(ChatContainerError::TargetPublicKeyNotFound)
            }
        }

        for public_key in participant_public_keys {
            if let Some(target_public_key) = self.participant_public_keys
                .iter()
                .position(|target| target.to_string() == public_key) {
                self.participant_public_keys.remove(target_public_key);
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChatContainerError {
    DirectChatCannotAddParticipants,
    DirectChatCannotRemoveParticipants,
    PermissionDenied,
    AlreadyExists,
    TargetPublicKeyNotFound
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChatContainerContext {
    Direct {
        other_public_key: String,
    },
    Group {
        group_type: ChatContainerGroupType,
        creator_public_key: String,
        admins_public_key: Vec<String>
    }
}

impl ChatContainerContext {
    fn is_direct(&self) -> bool {
        matches!(self, ChatContainerContext::Direct { .. })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChatContainerGroupType {
    Private,
    Channel
}
