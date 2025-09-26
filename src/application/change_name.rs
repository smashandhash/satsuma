use crate::domain::user::User;

pub struct ChangeNameUseCase;

impl ChangeNameUseCase {
    pub fn execute(&self, user: &mut User, new_name: &str) -> Result<(), String> {
        let trimmed_new_name = new_name.trim();
        if trimmed_new_name.is_empty() {
            return Err("New name cannot be empty".to_string());
        }

        user.change_name(trimmed_new_name);
        Ok(())
    }
}
