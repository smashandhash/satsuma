#[derive(Debug, Clone)]
pub struct User {
    pub id: u64,
    pub name: String
}

impl User {
    pub fn new(id: u64, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }

    pub fn change_name(&mut self, new_name: String) {
        self.name = new_name.to_string();
    }
}
