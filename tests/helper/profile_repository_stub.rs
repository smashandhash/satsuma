use satsuma::{
    domain::user::User,
    infrastructure::profile_repository::{
        ProfileRepository,
        ProfileRepositoryError,
    },
};

pub struct ProfileRepositoryStub {
    pub simulated_error: Option<ProfileRepositoryError>,
    pub mocked_user: Option<User>,
}

impl ProfileRepositoryStub {
    pub fn new(simulated_error: Option<ProfileRepositoryError>, mocked_user: Option<User>) -> Self {
        Self { simulated_error, mocked_user }
    }
}

impl ProfileRepository for ProfileRepositoryStub {
    fn load(&self, _public_key: String) -> Result<User, ProfileRepositoryError> {
        if let Some(err) = &self.simulated_error {
            Err(err.clone())
        } else {
            Ok(self.mocked_user.clone().expect("It supposed to either have a simulated_error or mocked_user"))
        }
    }
}

