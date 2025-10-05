use crate::domain::conversation::Conversation;

pub struct AddParticipantToConversationUseCase;

impl AddParticipantToConversationUseCase {
    pub fn execute(&self, conversation: &mut Conversation, actor_public_key: String, new_participant_public_key: String) -> Result<(), AddParticipantToConversationUseCaseError> {
        if conversation.creator_public_key  != actor_public_key {
            return Err(AddParticipantToConversationUseCaseError::RestrictedForCreator);
        }
        if conversation.participant_public_keys.contains(&new_participant_public_key) {
            return Err(AddParticipantToConversationUseCaseError::ParticipantAlreadyExist);
        }
        let _ = conversation.add_participant(new_participant_public_key);
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum AddParticipantToConversationUseCaseError {
    RestrictedForCreator,
    ParticipantAlreadyExist
}
