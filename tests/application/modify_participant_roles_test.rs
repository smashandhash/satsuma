#[cfg(test)]
mod tests {
    use satsuma::infrastructure::{
        local_storage::LocalStorage,
        user_repository::{
            UserRepository,
            UserRepositoryError,
        },
    };
    use crate::helper::{
        local_storage_stub::LocalStorageStub,
        user_repository_stub::UserRepositoryStub,
    };

    pub trait ModifyParticipantRolesUseCase {
        fn execute(&self, group_id: String, target_public_key: String, roles: Vec<String>, previous_event_id: Option<String>) -> Result<(), ModifyParticipantRolesUseCaseError>;
    }

    pub struct ModifyParticipantRolesUseCaseImplementation<S: LocalStorage, R: UserRepository> {
        pub storage: S,
        pub repository: R,
    }

    impl<S: LocalStorage, R: UserRepository> ModifyParticipantRolesUseCaseImplementation<S, R> {
        pub fn new(storage: S, repository: R) -> Self {
            Self {
                storage,
                repository,
            }
        }
    }

    impl<S: LocalStorage, R: UserRepository> ModifyParticipantRolesUseCase for ModifyParticipantRolesUseCaseImplementation<S, R> {
        fn execute(&self, group_id: String, target_public_key: String, roles: Vec<String>, previous_event_id: Option<String>) -> Result<(), ModifyParticipantRolesUseCaseError> {
            let assigner_user = self.storage.load_saved_user().map_err(|_| ModifyParticipantRolesUseCaseError::AssignerNotFound)?;

            if group_id.is_empty() {
                return Err(ModifyParticipantRolesUseCaseError::GroupIDEmpty)
            }

            if target_public_key.is_empty() {
                return Err(ModifyParticipantRolesUseCaseError::TargetPublicKeyEmpty)
            }

            if roles.len() == 0 {
                return Err(ModifyParticipantRolesUseCaseError::EmptyAssignedRoles)
            }

            for role in roles.clone() {
                if role.is_empty() {
                    return Err(ModifyParticipantRolesUseCaseError::RoleIsEmpty)
                }
            }

            Ok(self.repository.change_role(group_id, assigner_user.public_key, target_public_key, roles, previous_event_id).map_err(|e| ModifyParticipantRolesUseCaseError::RepositoryError(e))?) 
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum ModifyParticipantRolesUseCaseError {
        AssignerNotFound,
        GroupIDEmpty,
        TargetPublicKeyEmpty,
        EmptyAssignedRoles,
        RoleIsEmpty,
        RepositoryError(UserRepositoryError),
    }

    #[test]
    fn success_modify_participant_role() {
        let group_id = "group_id".to_string();
        let target_public_key = "target_public_key".to_string();
        let previous_event_id = "previous_event_id".to_string();
        let roles = vec!["Admin".to_string(), "Supervisor".to_string()];
        let local_storage = LocalStorageStub::new(None);
        let repository = UserRepositoryStub::new(None);
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
        let local_storage = LocalStorageStub::new(None);
        let repository = UserRepositoryStub::new(None);
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
        let local_storage = LocalStorageStub::new(None);
        let repository = UserRepositoryStub::new(None);
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
        let local_storage = LocalStorageStub::new(None);
        let repository = UserRepositoryStub::new(None);
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
        let local_storage = LocalStorageStub::new(None);
        let repository = UserRepositoryStub::new(None);
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
        let local_storage = LocalStorageStub::new(Some("Error Local Storage".to_string()));
        let repository = UserRepositoryStub::new(None);
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
        let local_storage = LocalStorageStub::new(None);
        let repository_error = UserRepositoryError::FailedToChangeUserRole;
        let repository = UserRepositoryStub::new(Some(repository_error.clone()));
        let sut = ModifyParticipantRolesUseCaseImplementation::new(local_storage, repository);

        let result = sut.execute(group_id, target_public_key, roles, Some(previous_event_id));

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ModifyParticipantRolesUseCaseError::RepositoryError(repository_error));
    }
}
