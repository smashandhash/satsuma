#[cfg(test)]
mod tests {
    use satsuma::domain::services::timestamp_validator::{
        TimestampValidator,
        DefaultTimestampValidator,
        TimestampValidatorError
    };
    use chrono::{ Duration, TimeZone, Utc };

    #[test]
    fn invalid_timestamp() {
        let created_at = i64::MAX as u64;
        let sut = DefaultTimestampValidator;
        let result = sut.validate_timestamp(created_at);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), TimestampValidatorError::InvalidTimestamp);
    }

    #[test]
    fn timestamp_too_far_in_the_future() {
        let now_timestamp = Utc::now().timestamp();
        let now_datetime = Utc.timestamp_opt(now_timestamp, 0).unwrap();
        let created_at_timestamp = now_datetime + Duration::seconds(60 * 6);
        let created_at = created_at_timestamp.timestamp() as u64;
        let sut = DefaultTimestampValidator;
        let result = sut.validate_timestamp(created_at);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), TimestampValidatorError::TimestampTooFarInTheFuture);
    }

    #[test]
    fn timestamp_too_old() {
        let now_timestamp = Utc::now().timestamp();
        let now_datetime = Utc.timestamp_opt(now_timestamp, 0).unwrap();
        let created_at_timestamp = now_datetime - Duration::days(8);
        let created_at = created_at_timestamp.timestamp() as u64;
        let sut = DefaultTimestampValidator;
        let result = sut.validate_timestamp(created_at);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), TimestampValidatorError::TimestampTooOld);
    }
}
