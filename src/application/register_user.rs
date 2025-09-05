use crate::domain::user::User;

pub struct RegisterUserUseCase {
    next_id: u64,
}

impl RegisterUserUseCase {
    pub fn new() -> Self {
        RegisterUserUseCase { next_id: 1 }
    }

    pub fn execute(&mut self, name: String) -> Result<User, String> {
        let clean_name = name.trim();
        if clean_name.is_empty() {
            return Err("User name cannot be empty".to_string());
        }

        let user = User::new(self.next_id, &clean_name);
        self.next_id += 1;
        
        Ok(user)
    }
}
