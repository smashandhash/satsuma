#[cfg(test)]
mod tests {
    use satsuma::application::remove_participant_from_conversation::RemoveParticipantFromConversationUseCase;
    use satsuma::domain::conversation::Conversation;
    use satsuma::domain::user::User;

    #[test]
    fn remove_existing_participant() {
        let creator = User::new(1, "Alice");
        let mut conversation = Conversation::new(1, creator.id, vec![1, 2, 3]);
        let use_case = RemoveParticipantFromConversationUseCase;

        let result = use_case.execute(&mut conversation, 1, 2);

        assert!(result.is_ok());
        assert_eq!(conversation.participant_ids, [1, 3]);
    }

    #[test]
    fn non_creator_unable_to_remove_existing_participant() {
        let creator = User::new(1, "Alice");
        let non_creator = User::new(2, "Bob");
        let mut conversation = Conversation::new(1, creator.id, vec![1, 2, 3]);
        let use_case = RemoveParticipantFromConversationUseCase;

        let result = use_case.execute(&mut conversation, non_creator.id, 3);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Only creator who can remove a participant.");
    }
}
