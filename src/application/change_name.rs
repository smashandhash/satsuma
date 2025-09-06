use crate::domain::user::User;

pub struct ChangeNameUseCase;

impl ChangeNameUseCase {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, user: &mut User, new_name: &str) {
        user.change_name(new_name);
    }
}
