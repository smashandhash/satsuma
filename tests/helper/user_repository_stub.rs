use satsuma::{
    domain::user::User,
    infrastructure::user_repository::{
        UserRepository,
        UserRepositoryError,
    },
};

pub struct UserRepositoryStub {
    pub simulated_error: Option<UserRepositoryError>,
}

impl UserRepositoryStub {
    pub fn new(simulated_error: Option<UserRepositoryError>) -> Self {
        Self { simulated_error }
    }
}

impl UserRepository for UserRepositoryStub {
    fn load(&self, _public_key: String) -> Result<User, UserRepositoryError> {
        self.simulated_error.clone().map_or(Ok(User::new("public_key", "name")), Err)
    }
    
    fn change_role(&self, _group_id: String, _assigner_public_key: String, _target_public_key: String, _roles: Vec<String>, _previous_event_id: Option<String>) -> Result<(), UserRepositoryError> {
        self.simulated_error.clone().map_or(Ok(()), Err)
    }
}

