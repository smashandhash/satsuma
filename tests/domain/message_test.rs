use satsuma::domain::message::Message;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_message_should_do_nothing() {
        let message = Message::new(1, 100, 200, "Hello!");
    }

    #[test]
    fn edit_message_should_edited() {
        let mut message = Message::new(1, 100, 200, "Hello");
        message.edit_content("Hello, world");
        assert_eq!(message.content, "Hello, world");
    }

    #[test]
    fn message_content_cannot_be_empty() {
        let cases = vec![
            (Message::new(2, 100, 200, ""), true),
            (Message::new(3, 100, 200, "Hi"), false),
        ];

        for (message, expected) in cases {
            assert_eq!(message.is_empty(), expected);
        }
    }

    #[test]
    fn message_can_check_if_to_self() {
        let cases = vec! [
            (Message::new(4, 101, 101, "Reminder"), true),
            (Message::new(4, 101, 202, "Hello"), false),
        ];

        for (message, expected) in cases {
            assert_eq!(message.is_to_self(), expected);
        }
    }
}
