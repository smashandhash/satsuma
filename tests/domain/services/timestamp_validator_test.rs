#[cfg(test)]
mod tests {
    use satsuma::domain::services::timestamp_validator::{
        TimestampValidator,
        DefaultTimestampValidator,
        TimestampValidatorError
    };

    #[test]
    fn invalid_timestamp() {
        let created_at = i64::MAX as u64;
        let sut = DefaultTimestampValidator;
        let result = sut.validate_timestamp(created_at);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), TimestampValidatorError::InvalidTimestamp);
    }
}
