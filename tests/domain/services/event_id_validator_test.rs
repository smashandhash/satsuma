#[cfg(test)]
mod tests {
    use satsuma::domain::event::Event;
    use satsuma::domain::services::event_id_validator::{
        EventIDValidator,
        Sha256EventIDValidator,
        EventIDValidatorError
    };
    use rstest::rstest;
    use crate::helper::generate_event_id::generate_event_id;

    #[rstest]
    #[case("Event ID's match", None, None)]
    #[case("Event ID's mismatch", Some("".to_string()), Some(EventIDValidatorError::EventIDMismatch))]
    fn event_id(
        #[case] _label: &str,
        #[case] mocked_id: Option<String>,
        #[case] expected_error: Option<EventIDValidatorError>
        ) {
        let public_key = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let created_at = 1700000000u64;
        let kind = 14u32;
        let tags: Vec<Vec<String>> = Vec::new();
        let content = "Hello, Bob".to_string();
        let mut id = generate_event_id(public_key, created_at, kind, &tags, &content);
        if let Some(mocked_value_id) = mocked_id {
            id = mocked_value_id;
        }
        let event = Event {
            id,
            public_key: public_key.to_string(), 
            created_at, 
            kind, 
            tags: tags.clone(), 
            content: content.clone(),
            signature: "signature".to_string()
        };
        let sut = Sha256EventIDValidator;
        let result = sut.validate_event_id(&event);

        if let Some(error) = expected_error {
            assert!(result.is_err());
            assert_eq!(result.unwrap_err(), error);
        } else {
            assert!(result.is_ok());
        }
    }
}
