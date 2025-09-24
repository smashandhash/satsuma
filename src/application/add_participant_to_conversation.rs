use crate::domain::conversation::Conversation;

pub struct AddParticipantToConversationUseCase;

impl AddParticipantToConversationUseCase {
    pub fn execute(&self, conversation: &mut Conversation, actor_id: u64, new_participant_id: u64) -> Result<(), String> {
        if conversation.creator_id  != actor_id {
            return Err("Only the creator can add participants".to_string());
        }
        conversation.add_participant(new_participant_id)
    }
}
