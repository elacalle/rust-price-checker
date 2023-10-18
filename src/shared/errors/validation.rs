use std::{
    error,
    fmt::{self},
};

#[derive(Debug)]
pub struct InvalidValue {
    message: String,
}

impl InvalidValue {
    pub fn new(message: &str) -> InvalidValue {
        InvalidValue {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for InvalidValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error {}", self.message)
    }
}

impl error::Error for InvalidValue {}
