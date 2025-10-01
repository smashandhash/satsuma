use crate::domain::conversation::Conversation;

pub struct RemoveParticipantFromConversationUseCase;

impl RemoveParticipantFromConversationUseCase {
    pub fn execute(&self, conversation: &mut Conversation, actor_id: u64, target_id: u64) -> Result<(), String> {
        if conversation.creator_id != actor_id {
            return Err("Only creator who can remove a participant.".to_string()); 
        }
        conversation.remove_participant(target_id);
        Ok(())
    }
}
