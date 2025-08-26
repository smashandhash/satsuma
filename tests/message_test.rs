use satsuma::domain::message::Message;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_does_nothing() {
        let message = Message::new(1, 100, 200, "Hello!");
    }

    #[test]
    fn edit_should_edited() {
        let mut message = Message::new(1, 100, 200, "Hello");
        message.edit_content("Hello, world");
        assert_eq!(message.content, "Hello, world");
    }
}
