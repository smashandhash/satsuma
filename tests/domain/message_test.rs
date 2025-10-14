#[cfg(test)]
mod tests {
    use satsuma::domain::message::Message;

    #[test]
    fn init_message_should_do_nothing() {
        let _message = Message::new(1, "npub100", "Hello!");
    }

    #[test]
    fn edit_message_should_edited() {
        let mut message = Message::new(1, "npub100", "Hello");
        message.edit_content("Hello, world");
        assert_eq!(message.content, "Hello, world");
    }

    #[test]
    fn message_content_cannot_be_empty() {
        let cases = vec![
            (Message::new(2, "npub100", ""), true),
            (Message::new(3, "npub100", "Hi"), false),
        ];

        for (message, expected) in cases {
            assert_eq!(message.is_empty(), expected);
        }
    }
}
