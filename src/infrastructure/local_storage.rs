use crate::domain::user::User;

pub trait LocalStorage {
    fn save_user(&self, user: &User) -> Result<(), String>;
}
