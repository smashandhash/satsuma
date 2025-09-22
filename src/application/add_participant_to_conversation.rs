use crate::domain::conversation::Conversation;

pub struct AddParticipantToConversationUseCase;

impl AddParticipantToConversationUseCase {
    pub fn execute(&self, conversation: &mut Conversation, actor_id: u64, new_participant_id: u64) -> Result<(), String> {
        conversation.add_participant(new_participant_id)
    }
}
