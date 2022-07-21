use std::fmt;

#[derive(Debug, PartialEq)]
pub enum LdError {
    EntryError,
    EmptyWordError,
}

impl std::error::Error for LdError {}

impl fmt::Display for LdError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LdError::EntryError => write!(f, "EntryError: No repeating identifier for words."),
            LdError::EmptyWordError => write!(f, "EmptyWordError: Word cannot be an empty string."),
        }
    }
}