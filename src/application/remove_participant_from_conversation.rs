use crate::domain::conversation::Conversation;

pub struct RemoveParticipantFromConversationUseCase;

impl RemoveParticipantFromConversationUseCase {
    pub fn execute(&self, conversation: &mut Conversation, actor_id: u64, target_id: u64) {
        conversation.remove_participant(target_id);
    }
}
