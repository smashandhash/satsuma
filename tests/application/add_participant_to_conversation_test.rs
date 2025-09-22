#[cfg(test)]
mod tests {
    use satsuma::application::add_participant_to_conversation::AddParticipantToConversationUseCase;
    use satsuma::domain::conversation::Conversation;
    use satsuma::domain::user::User;

    #[test]
    fn add_participant_to_conversation_by_creator() {
        let creator = User::new(1, "Alice");
        let existing_participant = User::new(2, "Bob");
        let mut conversation = Conversation::new(1, vec![creator.id, existing_participant.id]);

        let new_participant = User::new(3, "Chad");
        let use_case = AddParticipantToConversationUseCase;
        let participants = use_case.execute(&mut conversation, creator.id, new_participant.id);

        assert!(conversation.participant_ids.contains(&new_participant.id));
    }
}
