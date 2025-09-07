use crate::domain::user::User;

pub struct ChangeNameUseCase;

impl ChangeNameUseCase {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, user: &mut User, new_name: &str) -> Result<(), String> {
        if new_name.is_empty() {
            return Err("New name cannot be empty".to_string());
        }

        user.change_name(new_name);
        Ok(())
    }
}
