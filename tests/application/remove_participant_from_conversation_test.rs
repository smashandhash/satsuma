#[cfg(test)]
mod tests {
    use satsuma::{
        application::remove_participant_from_conversation::RemoveParticipantFromConversationUseCase,
        application::remove_participant_from_conversation::RemoveParticipantFromConversationUseCaseError,
        domain::conversation::Conversation,
        domain::user::User
    };

    #[test]
    fn remove_existing_participant() {
        let creator = User::new("npub1", "Alice");
        let mut conversation = Conversation::new(1, &creator.public_key, vec!["npub1", "npub2", "npub3"]);
        let use_case = RemoveParticipantFromConversationUseCase;

        let result = use_case.execute(&mut conversation, creator.public_key, "npub2".to_string());

        assert!(result.is_ok());
        assert_eq!(conversation.participant_public_keys, ["npub1", "npub3"]);
    }

    #[test]
    fn non_creator_unable_to_remove_existing_participant() {
        let creator = User::new("npub1", "Alice");
        let non_creator = User::new("npub2", "Bob");
        let mut conversation = Conversation::new(1, &creator.public_key, vec!["npub1", "npub2", "npub3"]);
        let use_case = RemoveParticipantFromConversationUseCase;

        let result = use_case.execute(&mut conversation, non_creator.public_key, "npub3".to_string());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), RemoveParticipantFromConversationUseCaseError::RestrictedForCreator);
    }
}
