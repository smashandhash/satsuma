use satsuma::{
    domain::user::User,
    infrastructure::profile_repository::{
        ProfileRepository,
        ProfileRepositoryError,
    },
};

pub struct ProfileRepositoryStub {
    pub simulated_error: Option<ProfileRepositoryError>,
}

impl ProfileRepositoryStub {
    pub fn new(simulated_error: Option<ProfileRepositoryError>) -> Self {
        Self { simulated_error }
    }
}

impl ProfileRepository for ProfileRepositoryStub {
    fn load(&self, _public_key: String) -> Result<User, ProfileRepositoryError> {
        self.simulated_error.clone().map_or(Ok(User::new("public_key", "name")), Err)
    }
}

