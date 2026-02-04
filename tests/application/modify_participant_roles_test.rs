#[cfg(test)]
mod tests {
    use satsuma::{
        application::modify_participant_roles::{
            ModifyParticipantRolesUseCase,
            ModifyParticipantRolesUseCaseImplementation,
            ModifyParticipantRolesUseCaseError,
        },
        infrastructure::user_repository::UserRepositoryError,
    };
    use crate::helper::{
        local_storage_stub::LocalStorageStub,
        user_repository_stub::UserRepositoryStub,
    };
    use std::sync::Arc;

    #[test]
    fn success_modify_participant_role() {
        let group_id = "group_id".to_string();
        let target_public_key = "target_public_key".to_string();
        let previous_event_id = "previous_event_id".to_string();
        let roles = vec!["Admin".to_string(), "Supervisor".to_string()];
        let local_storage = Arc::new(LocalStorageStub::new(None));
        let repository = Arc::new(UserRepositoryStub::new(None));
        let sut = ModifyParticipantRolesUseCaseImplementation::new(local_storage, repository);

        let result = sut.execute(group_id, target_public_key, roles, Some(previous_event_id));

        assert!(result.is_ok());
    }

    #[test]
    fn group_id_empty_failed_to_modify_participant_role() {
        let group_id = "".to_string();
        let target_public_key = "target_public_key".to_string();
        let previous_event_id = "previous_event_id".to_string();
        let roles = vec!["Admin".to_string(), "Supervisor".to_string()];
        let local_storage = Arc::new(LocalStorageStub::new(None));
        let repository = Arc::new(UserRepositoryStub::new(None));
        let sut = ModifyParticipantRolesUseCaseImplementation::new(local_storage, repository);

        let result = sut.execute(group_id, target_public_key, roles, Some(previous_event_id));

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ModifyParticipantRolesUseCaseError::GroupIDEmpty);
    }

    #[test]
    fn target_public_key_empty_failed_to_modify_participant_role() {
        let group_id = "group_id".to_string();
        let target_public_key = "".to_string();
        let previous_event_id = "previous_event_id".to_string();
        let roles = vec!["Admin".to_string(), "Supervisor".to_string()];
        let local_storage = Arc::new(LocalStorageStub::new(None));
        let repository = Arc::new(UserRepositoryStub::new(None));
        let sut = ModifyParticipantRolesUseCaseImplementation::new(local_storage, repository);

        let result = sut.execute(group_id, target_public_key, roles, Some(previous_event_id));

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ModifyParticipantRolesUseCaseError::TargetPublicKeyEmpty);
    }

    #[test]
    fn assigned_roles_empty_failed_to_modify_participant_role() {
        let group_id = "group_id".to_string();
        let target_public_key = "target_public_key".to_string();
        let previous_event_id = "previous_event_id".to_string();
        let roles = Vec::new();
        let local_storage = Arc::new(LocalStorageStub::new(None));
        let repository = Arc::new(UserRepositoryStub::new(None));
        let sut = ModifyParticipantRolesUseCaseImplementation::new(local_storage, repository);

        let result = sut.execute(group_id, target_public_key, roles, Some(previous_event_id));

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ModifyParticipantRolesUseCaseError::EmptyAssignedRoles);
    }

    #[test]
    fn some_role_is_empty_failed_to_modify_participant_role() {
        let group_id = "group_id".to_string();
        let target_public_key = "target_public_key".to_string();
        let previous_event_id = "previous_event_id".to_string();
        let roles = vec!["Admin".to_string(), "".to_string()];
        let local_storage = Arc::new(LocalStorageStub::new(None));
        let repository = Arc::new(UserRepositoryStub::new(None));
        let sut = ModifyParticipantRolesUseCaseImplementation::new(local_storage, repository);

        let result = sut.execute(group_id, target_public_key, roles, Some(previous_event_id));

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ModifyParticipantRolesUseCaseError::RoleIsEmpty);
    }

    #[test]
    fn local_storage_error_failed_to_modify_participant_role() {
        let group_id = "group_id".to_string();
        let target_public_key = "target_public_key".to_string();
        let previous_event_id = "previous_event_id".to_string();
        let roles = vec!["Admin".to_string(), "Supervisor".to_string()];
        let local_storage = Arc::new(LocalStorageStub::new(Some("Error Local Storage".to_string())));
        let repository = Arc::new(UserRepositoryStub::new(None));
        let sut = ModifyParticipantRolesUseCaseImplementation::new(local_storage, repository);

        let result = sut.execute(group_id, target_public_key, roles, Some(previous_event_id));

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ModifyParticipantRolesUseCaseError::AssignerNotFound);
    }

    #[test]
    fn repository_error_failed_to_modify_participant_role() {
        let group_id = "group_id".to_string();
        let target_public_key = "target_public_key".to_string();
        let previous_event_id = "previous_event_id".to_string();
        let roles = vec!["Admin".to_string(), "Supervisor".to_string()];
        let local_storage = Arc::new(LocalStorageStub::new(None));
        let repository_error = UserRepositoryError::FailedToChangeUserRole;
        let repository = Arc::new(UserRepositoryStub::new(Some(repository_error.clone())));
        let sut = ModifyParticipantRolesUseCaseImplementation::new(local_storage, repository);

        let result = sut.execute(group_id, target_public_key, roles, Some(previous_event_id));

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ModifyParticipantRolesUseCaseError::RepositoryError(repository_error));
    }
}
