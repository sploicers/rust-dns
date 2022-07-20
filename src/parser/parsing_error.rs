use std::io::{Error, ErrorKind, Result};

pub struct ParseError<T> {
    value: T,
}

impl<T> ParseError<T> {
    pub fn new(message: String) -> Result<T> {
        Err(Error::new(ErrorKind::InvalidData, message))
    }
}
