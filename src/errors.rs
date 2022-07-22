use std::fmt;

#[derive(Debug, PartialEq)]
pub enum LdError {
    RepeatingWordId,
    EmptyWord,
    BookDoesNotExist,
}

impl std::error::Error for LdError {}

impl fmt::Display for LdError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LdError::RepeatingWordId => write!(f,
                "RepeatingWordId: No repeating identifier for words."),
            LdError::EmptyWord => write!(f,
                "EmptyWord: Word cannot be an empty string."),
            LdError::BookDoesNotExist => write!(f,
                "BookDoesNotExist: Book with such BookId does not exist."),
        }
    }
}