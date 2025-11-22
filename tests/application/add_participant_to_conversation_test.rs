#[cfg(test)]
mod tests {
    use satsuma::application::add_participants_to_chat_container::{
        AddParticipantsToChatContainerUseCase,
        AddParticipantsToChatContainerUseCaseImplementation,
        // AddParticipantsToChatContainerUseCaseError,
    };
    use crate::helper::chat_container_repository_stub::ChatContainerRepositoryStub;
    // use rstest::rstest;

    /*
    #[rstest]
    #[case("creator adds new participant", User::new("npub1234", "Alice"), User::new("npub3124", "Chad"), vec!["npub1234", "npub2134"], true)]
    #[case("non-creator adds new participant", User::new("npub2134", "Bob"), User::new("npub3124", "Chad"), vec!["npub1234", "npub2134"], false)]
    #[case("creator adds existing participant", User::new("npub1234", "Alice"), User::new("npub2134", "Bob"), vec!["npub1234", "npub2134"], false)]
    #[case("non-creator adds existing participant", User::new("npub2134", "Bob"), User::new("npub1234", "Alice"), vec!["npub1234", "npub2134"], false)]
    #[case("creator adds self", User::new("npub1234", "Alice"), User::new("npub1234", "Alice"), vec!["npub1234", "npub2134"], false)]
    #[case("non-creator adds self", User::new("npub2134", "Bob"), User::new("npub2134", "Bob"), vec!["npub1234", "npub2134"], false)]
    */
    #[test]
    fn add_participant_to_conversation(
        /*
        #[case] _label: &str,
        #[case] actor: User,
        #[case] new_participant: User,
        #[case] participant_public_keys: Vec<&str>,
        #[case] should_succeed: bool,
        */
        ) {
        /*
        let creator = User::new("npub1234", "Alice");
        let mut conversation = Conversation::new(1, &creator.public_key, participant_public_keys);
        */
        let repository = ChatContainerRepositoryStub {
            simulated_error: None,
            mocked_chat_container: None,
        };
        let sut = AddParticipantsToChatContainerUseCaseImplementation::new(repository);

        let result = sut.execute(
            "chat_container_id".to_string(), 
            "actor_public_key".to_string(), 
            Vec::new()
        );

        assert!(result.is_ok());
    }
}
