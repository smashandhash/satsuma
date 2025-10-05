#[derive(Debug, Clone)]
pub struct User {
    pub public_key: String,
    pub name: String
}

impl User {
    pub fn new(public_key: &str, name: &str) -> Self {
        Self {
            public_key: public_key.to_string(),
            name: name.to_string(),
        }
    }

    pub fn change_name(&mut self, new_name: String) {
        self.name = new_name.to_string();
    }
}
