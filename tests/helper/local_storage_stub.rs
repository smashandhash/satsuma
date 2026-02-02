use satsuma::{
    domain::user::User,
    infrastructure::local_storage::LocalStorage
};

pub struct LocalStorageStub {
    pub simulated_error: Option<String>
}

impl LocalStorageStub {
    pub fn new(simulated_error: Option<String>) -> Self {
        Self { simulated_error }
    }
}

impl LocalStorage for LocalStorageStub {
    fn save_user(&self, _user: &User) -> Result<(), String> {
        if let Some(simulated_error) = &self.simulated_error {
            Err(simulated_error.clone())
        } else {
            Ok(())
        }
    }

    fn save_secret_key(&self, _secret_key: &String) -> Result<(), String> {
        Ok(())
    }

    fn load_secret_key(&self) -> Result<String, String> {
        if let Some(simulated_error) = &self.simulated_error {
            Err(simulated_error.clone())
        } else {
            Ok("Secret Key".to_string())
        }
    }

    fn load_saved_user(&self) -> Result<User, String> {
        if let Some(simulated_error) = &self.simulated_error {
            Err(simulated_error.clone())
        } else {
            Ok(User::new("public_key", "name"))
        }
    }
}
