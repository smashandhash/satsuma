#[cfg(test)]
mod tests {
    use satsuma::{
        application::add_participant_to_conversation::AddParticipantToConversationUseCase,
        domain::conversation::Conversation,
        domain::user::User
    };
    use rstest::rstest;

    #[rstest]
    #[case("creator adds new participant", User::new(1, "Alice"), User::new(3, "Chad"), vec![1, 2], true)]
    #[case("non-creator adds new participant", User::new(2, "Bob"), User::new(3, "Chad"), vec![1, 2], false)]
    #[case("creator adds existing participant", User::new(1, "Alice"), User::new(2, "Bob"), vec![1, 2], false)]
    #[case("non-creator adds existing participant", User::new(2, "Bob"), User::new(1, "Alice"), vec![1, 2], false)]
    #[case("creator adds self", User::new(1, "Alice"), User::new(1, "Alice"), vec![1, 2], false)]
    #[case("non-creator adds self", User::new(2, "Bob"), User::new(2, "Bob"), vec![1, 2], false)]
    fn add_participant_to_conversation(
        #[case] _label: &str,
        #[case] actor: User,
        #[case] new_participant: User,
        #[case] participant_ids: Vec<u64>,
        #[case] should_succeed: bool,
        ) {
        let creator = User::new(1, "Alice");
        let mut conversation = Conversation::new(1, creator.id, participant_ids);
        let use_case = AddParticipantToConversationUseCase;

        let result = use_case.execute(&mut conversation, actor.id, new_participant.id);

        assert_eq!(result.is_ok(), should_succeed);
    }
}
