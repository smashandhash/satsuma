#[cfg(test)]
mod tests {
    use satsuma::domain::event::Event;
    use satsuma::domain::services::event_id_validator::{
        EventIDValidator,
        Sha256EventIDValidator,
        EventIDValidatorError
    };
    use chrono::Utc;

    #[test]
    fn event_id_mismatch() {
        let public_key = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let created_at = Utc::now().timestamp() as u64;
        let kind = 14 as u32;
        let event_id = Event {
            id: "".to_string(),
            public_key: public_key.to_string(), 
            created_at, 
            kind, 
            tags: Vec::new(), 
            content: "Hello, Bob.".to_string(),
            signature: "signature".to_string()
        };
        let sut = Sha256EventIDValidator;
        let result = sut.validate_event_id(&event_id);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), EventIDValidatorError::EventIDMismatch);
    }
}
