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

        use_case.execute(&mut conversation, 1, 2);

        assert_eq!(conversation.participant_ids, [1, 3]);
    }
}
