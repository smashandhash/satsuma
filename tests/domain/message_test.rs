#[cfg(test)]
mod tests {
    use satsuma::domain::{
        message::Message,
        event_kind::EventKind
    };

    #[test]
    fn init_message_should_do_nothing() {
        let _message = Message::new("npub100", "Hello!", EventKind::DirectMessage);
    }

    #[test]
    fn edit_message_should_edited() {
        let mut message = Message::new("npub100", "Hello", EventKind::DirectMessage);
        message.edit_content("Hello, world");
        assert_eq!(message.content, "Hello, world");
    }

    #[test]
    fn message_content_cannot_be_empty() {
        let cases = vec![
            (Message::new("npub100", "", EventKind::DirectMessage), true),
            (Message::new("npub100", "Hi", EventKind::DirectMessage), false),
        ];

        for (message, expected) in cases {
            assert_eq!(message.is_empty(), expected);
        }
    }
}
