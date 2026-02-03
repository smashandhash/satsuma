use crate::infrastructure::{
    local_storage::LocalStorage,
    user_repository::{
        UserRepository,
        UserRepositoryError,
    },
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
