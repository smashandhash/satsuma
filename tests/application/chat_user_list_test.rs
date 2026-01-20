#[cfg(test)]
mod tests {
    use satsuma::domain::{
        user::User,
        chat_container::ChatContainer,
    };

    pub trait GetChatUserListUseCase {
        fn execute(&self, chat_container_id: String) -> Vec<User>;
    }

    fn get_chat_user_list() {
    }
}
