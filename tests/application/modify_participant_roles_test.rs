#[cfg(test)]
mod tests {
    use satsuma::{
        domain::{
            user::User,
            chat_container::ChatContainer,
        },
    };

    pub trait ModifyParticipantRolesUseCase {
        // TODO: Decide the Role, either it's an enum or a regular String.
        fn execute(&self, group_id: String, public_key: String, previous_event_id: Option<String>);
    }

    pub struct ModifyParticipantRolesUseCaseImplementation;

    impl ModifyParticipantRolesUseCase for ModifyParticipantRolesUseCaseImplementation {
        fn execute(&self, group_id: String, public_key: String, previous_event_id: Option<String>) {
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
}
