pub fn validate_kind(kind: u32) -> Result<(), ValidateKindError> {
    match kind {
        0 => Ok(()),
        14 => Ok(()), 
        42 => Ok(()),
        _ => Err(ValidateKindError::InvalidKindValue(kind))
    }
}

#[derive(Debug, PartialEq)]
pub enum ValidateKindError {
    InvalidKindValue(u32)
}
