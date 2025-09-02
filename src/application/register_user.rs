use crate::domain::user::User;

pub struct RegisterUserUseCase {
    next_id: u64,
}

impl RegisterUserUseCase {
    pub fn new() -> Self {
        RegisterUserUseCase { next_id: 1 }
    }

    pub fn execute(&mut self, name: String) -> User {
        let user = User::new(self.next_id, &name);
        self.next_id += 1;
        user
    }
}
