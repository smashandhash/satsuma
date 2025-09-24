#[cfg(test)]
mod tests {
    use satsuma::application::add_participant_to_conversation::AddParticipantToConversationUseCase;
    use satsuma::domain::conversation::Conversation;
    use satsuma::domain::user::User;

    #[test]
    fn add_participant_to_conversation_by_creator() {
        let creator = User::new(1, "Alice");
        let existing_participant = User::new(2, "Bob");
        let mut conversation = Conversation::new(1, creator.id, vec![creator.id, existing_participant.id]);

        let new_participant = User::new(3, "Chad");
        let use_case = AddParticipantToConversationUseCase;
        let participants = use_case.execute(&mut conversation, creator.id, new_participant.id);

        assert!(conversation.participant_ids.contains(&new_participant.id));
    }

    #[test]
    fn add_participant_to_conversation_failed_by_non_creator() {
        let non_creator = User::new(2, "Bob");
        let creator = User::new(1, "Alice");
        let mut conversation = Conversation::new(1, creator.id, vec![creator.id, non_creator.id]);

        let new_participant = User::new(3, "Chad");
        let use_case = AddParticipantToConversationUseCase;
        let result = use_case.execute(&mut conversation, non_creator.id, new_participant.id);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Only the creator can add participants");
    }
}
