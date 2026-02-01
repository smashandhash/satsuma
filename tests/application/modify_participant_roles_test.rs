#[cfg(test)]
mod tests {
    use satsuma::{
        domain::{
            user::User,
            chat_container::ChatContainer,
        },
        infrastructure::local_storage::LocalStorage,
    };

    pub trait ModifyParticipantRolesUseCase {
        // TODO: Decide the Role, either it's an enum or a regular String.
        fn execute(&self, group_id: String, target_public_key: String, previous_event_id: Option<String>) -> Result<(), ModifyParticipantRolesUseCaseError>;
    }

    pub struct ModifyParticipantRolesUseCaseImplementation<S: LocalStorage> {
        pub storage: S,
    }

    impl<S: LocalStorage> ModifyParticipantRolesUseCaseImplementation<S> {
        pub fn new(storage: S) -> Self {
            Self {
                storage,
            }
        }
    }

    impl<S: LocalStorage> ModifyParticipantRolesUseCase for ModifyParticipantRolesUseCaseImplementation<S> {
        fn execute(&self, group_id: String, target_public_key: String, previous_event_id: Option<String>) -> Result<(), ModifyParticipantRolesUseCaseError> {
            let public_key = self.storage.load_saved_user().unwrap();
            if group_id.is_empty() {
                return Err(ModifyParticipantRolesUseCaseError::GroupIDEmpty);
            }

            if public_key.is_empty() {
                return Err(ModifyParticipantRolesUseCaseError::AssignerPublicKeyEmpty);
            }
            // TODO: Set the user's public key who do this thing
            // TODO: Kind is 9000
            // TODO: Create tags variable with 3 properties
            // TODO: 1. "h" for group_id
            // TODO: 2. "p" for target's public key
            // TODO: 3. "previous" for an optional previous event_id, but it's recommended to have
            // it.
            // TODO: Set the content into fixed value of "Modify a participant of `target's public_key` into a role of `target_role`"
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum ModifyParticipantRolesUseCaseError {
        GroupIDEmpty,
        AssignerPublicKeyEmpty,
        TargetPublicKeyEmpty,
    }

    #[test]
    fn success_modify_participant_role() {
        let group_id = "group_id".to_string();
        let user_public_key = "user_public_key".to_string();
        let target_public_key = "target_public_key".to_string();
        let previous_event_id = "previous_event_id".to_string();
        let sut = ModifyParticipantRolesUseCaseImplementation;

        sut.execute(group_id, user_public_key, target_public_key, Some(previous_event_id));
    }
}
