use crate::domain::user::User;

pub struct RegisterUserUseCase {
    next_id: u64,
}

impl RegisterUserUseCase {
    pub fn new() -> Self {
        RegisterUserUseCase { next_id: 1 }
    }

    pub fn execute(&mut self, name: String) -> Result<User, String> {
        if name.trim().is_empty() {
            return Err("User name cannot be empty".to_string());
        }

        let user = User::new(&self.next_id.to_string(), &name.trim());
        self.next_id += 1;
        
        Ok(user)
    }
}
