#[cfg(test)]
mod tests {
    use satsuma::domain::services::timestamp_validator::{
        TimestampValidator,
        DefaultTimestampValidator,
        TimestampValidatorError
    };
    use chrono::{ Duration, TimeZone, Utc };
    use rstest::rstest;

    #[test]
    fn invalid_timestamp() {
        let created_at = i64::MAX as u64;
        let sut = DefaultTimestampValidator;
        let result = sut.validate_timestamp(created_at);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), TimestampValidatorError::InvalidTimestamp);
    }

    #[rstest]
    #[case("Too far in the future", Duration::seconds(60 * 6), true, TimestampValidatorError::TimestampTooFarInTheFuture)]
    #[case("Too old", Duration::days(8), false, TimestampValidatorError::TimestampTooOld)]
    fn timestamp_validity_error(
        #[case] _label: &str,
        #[case] duration: Duration,
        #[case] is_future: bool,
        #[case] expected_error: TimestampValidatorError) {
        let now_timestamp = Utc::now().timestamp();
        let now_datetime = Utc.timestamp_opt(now_timestamp, 0).unwrap();
        let created_at_timestamp = if is_future { now_datetime + duration } else { now_datetime - duration };
        let created_at = created_at_timestamp.timestamp() as u64;
        let sut = DefaultTimestampValidator;
        let result = sut.validate_timestamp(created_at);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), expected_error);
    }
}
