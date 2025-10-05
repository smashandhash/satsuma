use crate::domain::conversation::Conversation;

pub struct RemoveParticipantFromConversationUseCase;

impl RemoveParticipantFromConversationUseCase {
    pub fn execute(&self, conversation: &mut Conversation, actor_public_key: String, target_public_key: String) -> Result<(), RemoveParticipantFromConversationUseCaseError> {
        if conversation.creator_public_key != actor_public_key {
            return Err(RemoveParticipantFromConversationUseCaseError::RestrictedForCreator); 
        }
        conversation.remove_participant(target_public_key);
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum RemoveParticipantFromConversationUseCaseError {
    RestrictedForCreator
}
