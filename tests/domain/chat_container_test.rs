#[cfg(test)]
mod tests {
    use satsuma::domain::chat_container::{
        ChatContainer,
        ChatContainerError,
        ChatContainerContext,
        ChatContainerGroupType
    };

    #[test]
    fn init_should_do_nothing() {
        let _conversation = ChatContainer::new("id".to_string(), 
            ChatContainerContext::Direct("recipient_public_key"), 
            Vec::new(),
            Vec::new());
    }

    #[test]
    fn can_add_participants() {
        let actor_public_key = "actor_public_key".to_string();
        let mut sut = ChatContainer::new("id".to_string(), 
            ChatContainerContext::Group(
                ChatContainerGroupType::Private, 
                actor_public_key, 
                vec![actor_public_key]
                ), 
            vec![actor_public_key],
            Vec::new());
        let new_participant_public_keys = vec!["new_public_key".to_string()];

        let result = sut.add_participants(&actor_public_key, new_participant_public_keys);

        assert!(result.is_ok());
        assert_eq!(sut.participant_public_keys.len(), 2);
    }

    #[test]
    fn can_remove_participants() {
        let actor_public_key = "actor_public_key".to_string();
        let target_public_key = "target_public_key".to_string();
        let mut sut = ChatContainer::new("id".to_string(), 
            ChatContainerContext::Group(
                ChatContainerGroupType::Channel, 
                actor_public_key, 
                vec![actor_public_key]
                ), 
            vec![actor_public_key, target_public_key],
            Vec::new());
        let participant_public_keys = vec![target_public_key];

        let result = sut.remove_participants(&actor_public_key, participant_public_keys);

        assert!(result.is_ok());
        assert_eq!(sut.participant_public_keys.len(), 1);
    }

    #[test]
    fn direct_chat_cannot_add_participants() {
        let actor_public_key = "actor_public_key".to_string();
        let target_public_key = "target_public_key".to_string();
        let mut sut = ChatContainer::new("id".to_string(), 
            ChatContainerContext::Direct(target_public_key), 
            vec![actor_public_key, target_public_key],
            Vec::new());
        let participant_public_keys = vec![target_public_key];

        let result = sut.add_participants(&actor_public_key, participant_public_keys);

        assert!(result.is_err());
        assert_eq!(sut.unwrap_err(), ChatContainerError::DirectChatCannotAddParticipants);
    }

    #[test]
    fn direct_chat_cannot_remove_participants() {
        let actor_public_key = "actor_public_key".to_string();
        let target_public_key = "target_public_key".to_string();
        let mut sut = ChatContainer::new("id".to_string(), 
            ChatContainerContext::Direct(target_public_key), 
            vec![actor_public_key, target_public_key],
            Vec::new());
        let participant_public_keys = vec![target_public_key];

        let result = sut.remove_participants(&actor_public_key, participant_public_keys);

        assert!(result.is_err());
        assert_eq!(sut.unwrap_err(), ChatContainerError::DirectChatCannotAddParticipants);
    }

    #[test]
    fn permission_denied_cannot_add_participants() {
        let admin_public_key = "admin_public_key".to_string();
        let actor_public_key = "actor_public_key".to_string();
        let target_public_key = "target_public_key".to_string();
        let mut sut = ChatContainer::new("id".to_string(), 
            ChatContainerContext::Group(
                ChatContainerGroupType::Private, 
                admin_public_key, 
                vec![admin_public_key]
                ), 
            vec![actor_public_key],
            Vec::new());

        let participant_public_keys = vec![target_public_key];

        let result = sut.add_participants(&actor_public_key, participant_public_keys);

        assert!(result.is_err());
        assert_eq!(sut.unwrap_err(), ChatContainerError::PermissionDenied);
    }

    #[test]
    fn permission_denied_cannot_remove_participants() {
        let admin_public_key = "admin_public_key".to_string();
        let actor_public_key = "actor_public_key".to_string();
        let target_public_key = "target_public_key".to_string();
        let mut sut = ChatContainer::new("id".to_string(), 
            ChatContainerContext::Group(
                ChatContainerGroupType::Private, 
                admin_public_key, 
                vec![admin_public_key]
                ), 
            vec![actor_public_key, target_public_key],
            Vec::new());

        let participant_public_keys = vec![target_public_key];

        let result = sut.remove_participants(&actor_public_key, participant_public_keys);

        assert!(result.is_err());
        assert_eq!(sut.unwrap_err(), ChatContainerError::PermissionDenied);
    }

    #[test]
    fn already_exists_cannot_add_participants() {
        let admin_public_key = "admin_public_key".to_string();
        let target_public_key = "target_public_key".to_string();
        let mut sut = ChatContainer::new("id".to_string(), 
            ChatContainerContext::Group(
                ChatContainerGroupType::Private, 
                admin_public_key, 
                vec![admin_public_key]
                ), 
            vec![admin_public_key, target_public_key],
            Vec::new());

        let participant_public_keys = vec![target_public_key];

        let result = sut.add_participants(&admin_public_key, participant_public_keys);

        assert!(result.is_err());
        assert_eq!(sut.unwrap_err(), ChatContainerError::AlreadyExists);
    }

    #[test]
    fn target_public_key_not_found_cannot_remove_participants() {
        let admin_public_key = "admin_public_key".to_string();
        let target_public_key = "target_public_key".to_string();
        let mut sut = ChatContainer::new("id".to_string(), 
            ChatContainerContext::Group(
                ChatContainerGroupType::Private, 
                admin_public_key, 
                vec![admin_public_key]
                ), 
            vec![admin_public_key],
            Vec::new());

        let participant_public_keys = vec![target_public_key];

        let result = sut.remove_participants(&admin_public_key, participant_public_keys);

        assert!(result.is_err());
        assert_eq!(sut.unwrap_err(), ChatContainerError::TargetPublicKeyNotFound);
    }
}
