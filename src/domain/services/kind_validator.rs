pub trait KindValidator {
    fn validate_kind(&self, kind: u32) -> Result<(), KindValidatorError>;
}

pub struct DefaultKindValidator;

impl KindValidator for DefaultKindValidator {
    fn validate_kind(&self, kind: u32) -> Result<(), KindValidatorError> {
        match kind {
            0 => Ok(()),
            14 => Ok(()), 
            42 => Ok(()),
            _ => Err(KindValidatorError::InvalidKindValue(kind))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KindValidatorError {
    InvalidKindValue(u32)
}
