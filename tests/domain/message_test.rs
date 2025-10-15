#[cfg(test)]
mod tests {
    use satsuma::domain::{
        message::Message,
        event_kind::EventKind
    };

    #[test]
    fn init_message_should_do_nothing() {
        let _message = Message::new("npub100", "Hello!", EventKind::PrivateOrGroupMessage, Vec::new());
    }

    #[test]
    fn edit_message_should_edited() {
        let mut message = Message::new("npub100", "Hello", EventKind::PrivateOrGroupMessage, Vec::new());
        message.edit_content("Hello, world");
        assert_eq!(message.content, "Hello, world");
    }

    #[test]
    fn message_content_cannot_be_empty() {
        let cases = vec![
            (Message::new("npub100", "", EventKind::PrivateOrGroupMessage, Vec::new()), true),
            (Message::new("npub100", "Hi", EventKind::PrivateOrGroupMessage, Vec::new()), false),
        ];

        for (message, expected) in cases {
            assert_eq!(message.is_empty(), expected);
        }
    }
}
