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
            ChatContainerContext::Direct { other_public_key: "recipient_public_key".to_string() }, 
            Vec::new(),
            Vec::new());
    }

    #[test]
    fn can_add_participants() {
        let actor_public_key = "actor_public_key".to_string();
        let mut sut = ChatContainer::new("id".to_string(), 
            ChatContainerContext::Group {
                group_type: ChatContainerGroupType::Private, 
                creator_public_key: actor_public_key.clone(), 
                admins_public_key: vec![actor_public_key.clone()]
            }, 
            vec![actor_public_key.clone()],
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
            ChatContainerContext::Group {
                group_type: ChatContainerGroupType::Channel, 
                creator_public_key: actor_public_key.clone(), 
                admins_public_key: vec![actor_public_key.clone()]
            }, 
            vec![actor_public_key.clone(), target_public_key.clone()],
            Vec::new());
        let participant_public_keys = vec![target_public_key.clone()];

        let result = sut.remove_participants(&actor_public_key, participant_public_keys);

        assert!(result.is_ok());
        assert_eq!(sut.participant_public_keys.len(), 1);
    }

    #[test]
    fn direct_chat_cannot_add_participants() {
        let actor_public_key = "actor_public_key".to_string();
        let target_public_key = "target_public_key".to_string();
        let mut sut = ChatContainer::new("id".to_string(), 
            ChatContainerContext::Direct { other_public_key: target_public_key.clone() }, 
            vec![actor_public_key.clone(), target_public_key.clone()],
            Vec::new());
        let participant_public_keys = vec![target_public_key.clone()];

        let result = sut.add_participants(&actor_public_key, participant_public_keys);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ChatContainerError::DirectChatCannotAddParticipants);
    }

    #[test]
    fn direct_chat_cannot_remove_participants() {
        let actor_public_key = "actor_public_key".to_string();
        let target_public_key = "target_public_key".to_string();
        let mut sut = ChatContainer::new("id".to_string(), 
            ChatContainerContext::Direct { other_public_key: target_public_key.clone() }, 
            vec![actor_public_key.clone(), target_public_key.clone()],
            Vec::new());
        let participant_public_keys = vec![target_public_key.clone()];

        let result = sut.remove_participants(&actor_public_key, participant_public_keys);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ChatContainerError::DirectChatCannotAddParticipants);
    }

    #[test]
    fn permission_denied_cannot_add_participants() {
        let admin_public_key = "admin_public_key".to_string();
        let actor_public_key = "actor_public_key".to_string();
        let target_public_key = "target_public_key".to_string();
        let mut sut = ChatContainer::new("id".to_string(), 
            ChatContainerContext::Group {
                group_type: ChatContainerGroupType::Private, 
                creator_public_key: admin_public_key.clone(), 
                admins_public_key: vec![admin_public_key.clone()]
            }, 
            vec![actor_public_key.clone()],
            Vec::new());

        let participant_public_keys = vec![target_public_key.clone()];

        let result = sut.add_participants(&actor_public_key, participant_public_keys);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ChatContainerError::PermissionDenied);
    }

    #[test]
    fn permission_denied_cannot_remove_participants() {
        let admin_public_key = "admin_public_key".to_string();
        let actor_public_key = "actor_public_key".to_string();
        let target_public_key = "target_public_key".to_string();
        let mut sut = ChatContainer::new("id".to_string(), 
            ChatContainerContext::Group {
                group_type: ChatContainerGroupType::Private, 
                creator_public_key: admin_public_key.clone(), 
                admins_public_key: vec![admin_public_key.clone()]
            }, 
            vec![actor_public_key.clone(), target_public_key.clone()],
            Vec::new());

        let participant_public_keys = vec![target_public_key.clone()];

        let result = sut.remove_participants(&actor_public_key, participant_public_keys);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ChatContainerError::PermissionDenied);
    }

    #[test]
    fn already_exists_cannot_add_participants() {
        let admin_public_key = "admin_public_key".to_string();
        let target_public_key = "target_public_key".to_string();
        let mut sut = ChatContainer::new("id".to_string(), 
            ChatContainerContext::Group {
                group_type: ChatContainerGroupType::Private, 
                creator_public_key: admin_public_key.clone(), 
                admins_public_key: vec![admin_public_key.clone()]
            }, 
            vec![admin_public_key.clone(), target_public_key.clone()],
            Vec::new());

        let participant_public_keys = vec![target_public_key.clone()];

        let result = sut.add_participants(&admin_public_key, participant_public_keys);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ChatContainerError::AlreadyExists);
    }

    #[test]
    fn target_public_key_not_found_cannot_remove_participants() {
        let admin_public_key = "admin_public_key".to_string();
        let target_public_key = "target_public_key".to_string();
        let mut sut = ChatContainer::new("id".to_string(), 
            ChatContainerContext::Group {
                group_type: ChatContainerGroupType::Private, 
                creator_public_key: admin_public_key.clone(), 
                admins_public_key: vec![admin_public_key.clone()]
            }, 
            vec![admin_public_key.clone()],
            Vec::new());

        let participant_public_keys = vec![target_public_key.clone()];

        let result = sut.remove_participants(&admin_public_key, participant_public_keys);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ChatContainerError::TargetPublicKeyNotFound);
    }
}
