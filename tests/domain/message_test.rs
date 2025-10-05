#[cfg(test)]
mod tests {
    use satsuma::domain::message::Message;

    #[test]
    fn init_message_should_do_nothing() {
        let _message = Message::new(1, "npub100", "npub200", "Hello!");
    }

    #[test]
    fn edit_message_should_edited() {
        let mut message = Message::new(1, "npub100", "npub200", "Hello");
        message.edit_content("Hello, world");
        assert_eq!(message.content, "Hello, world");
    }

    #[test]
    fn message_content_cannot_be_empty() {
        let cases = vec![
            (Message::new(2, "npub100", "npub200", ""), true),
            (Message::new(3, "npub100", "npub200", "Hi"), false),
        ];

        for (message, expected) in cases {
            assert_eq!(message.is_empty(), expected);
        }
    }

    #[test]
    fn message_can_check_if_to_self() {
        let cases = vec! [
            (Message::new(4, "npub101", "npub101", "Reminder"), true),
            (Message::new(4, "npub101", "npub202", "Hello"), false),
        ];

        for (message, expected) in cases {
            assert_eq!(message.is_to_self(), expected);
        }
    }
}
