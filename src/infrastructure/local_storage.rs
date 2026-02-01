use crate::domain::user::User;

pub trait LocalStorage {
    fn save_user(&self, user: &User) -> Result<(), String>;
    fn save_secret_key(&self, secret_key: &String) -> Result<(), String>;
    fn load_secret_key(&self) -> Result<String, String>;
    fn load_saved_user() -> Result<Option<User>, String>;
}
