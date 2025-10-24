use chrono::{Utc, TimeZone, Duration};

pub trait TimestampValidator{
    fn validate_timestamp(&self, created_at: u64) -> Result<(), TimestampValidatorError>;
}

pub struct DefaultTimestampValidator;

impl TimestampValidator for DefaultTimestampValidator {
    fn validate_timestamp(&self, created_at: u64) -> Result<(), TimestampValidatorError> {
        let now = Utc::now().timestamp() as i64;

        if created_at > i64::MAX as u64 {
            return Err(TimestampValidatorError::InvalidTimestamp)
        }

        let created_at_i64 = created_at as i64;
        let max_future_offset = Duration::seconds(60 * 5);
        let max_past_offset = Duration::days(7);

        let message_time = Utc.timestamp_opt(created_at_i64, 0).single().ok_or(TimestampValidatorError::InvalidTimestamp)?;

        let now_time = Utc.timestamp_opt(now, 0).unwrap();

        if message_time > now_time + max_future_offset {
            return Err(TimestampValidatorError::TimestampTooFarInTheFuture);
        }

        if message_time < now_time - max_past_offset {
            return Err(TimestampValidatorError::TimestampTooOld);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TimestampValidatorError {
    InvalidTimestamp,
    TimestampTooFarInTheFuture,
    TimestampTooOld
}
