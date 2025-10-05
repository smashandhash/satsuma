#[cfg(test)]
mod tests {
    use satsuma::{
        application::add_participant_to_conversation::AddParticipantToConversationUseCase,
        domain::conversation::Conversation,
        domain::user::User
    };
    use rstest::rstest;

    #[rstest]
    #[case("creator adds new participant", User::new("npub1234", "Alice"), User::new("npub3124", "Chad"), vec!["npub1234", "npub2134"], true)]
    #[case("non-creator adds new participant", User::new("npub2134", "Bob"), User::new("npub3124", "Chad"), vec!["npub1234", "npub2134"], false)]
    #[case("creator adds existing participant", User::new("npub1234", "Alice"), User::new("npub2134", "Bob"), vec!["npub1234", "npub2134"], false)]
    #[case("non-creator adds existing participant", User::new("npub2134", "Bob"), User::new("npub1234", "Alice"), vec!["npub1234", "npub2134"], false)]
    #[case("creator adds self", User::new("npub1234", "Alice"), User::new("npub1234", "Alice"), vec!["npub1234", "npub2134"], false)]
    #[case("non-creator adds self", User::new("npub2134", "Bob"), User::new("npub2134", "Bob"), vec!["npub1234", "npub2134"], false)]
    fn add_participant_to_conversation(
        #[case] _label: &str,
        #[case] actor: User,
        #[case] new_participant: User,
        #[case] participant_public_keys: Vec<&str>,
        #[case] should_succeed: bool,
        ) {
        let creator = User::new("npub1234", "Alice");
        let mut conversation = Conversation::new(1, &creator.public_key, participant_public_keys);
        let use_case = AddParticipantToConversationUseCase;

        let result = use_case.execute(&mut conversation, actor.public_key, new_participant.public_key);

        assert_eq!(result.is_ok(), should_succeed);
    }
}
