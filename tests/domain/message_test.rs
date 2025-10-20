#[cfg(test)]
mod tests {
    use satsuma::domain::{
        message::Message,
        event_kind::EventKind
    };
    use chrono::Utc;
    use rstest::rstest;
    use crate::helper::generate_event_id::generate_event_id;

    #[rstest]
    #[case("message successfully edited", "npub100", "Hello", None, false)]
    #[case("message's content cannot be empty", "npub100", "", None, true)]
    #[case("message's content edited successfully", "npub100", "Hi", Some("Hello".to_string()), false)]
    fn message_content(
        #[case] _label: &str,
        #[case] public_key: String,
        #[case] content: String,
        #[case] new_content: Option<String>,
        #[case] is_empty: bool) {
        let created_at = Utc::now().timestamp() as u64;
        let kind = EventKind::PrivateOrGroupMessage;
        let id = generate_event_id(&public_key, created_at.clone(), kind.clone() as u32, &Vec::new(), &content);
        let mut message = Message::new(id, public_key, content, created_at.clone(), kind.clone(), Vec::new());

        if let Some(new_content) = new_content {
            message.edit_content(&new_content);
        }

        assert_eq!(message.is_empty(), is_empty);
    }
}
