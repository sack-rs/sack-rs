use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum SackError {
    SackNotFound,
}

impl fmt::Display for SackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.description())
    }
}

impl Error for SackError {
    fn description(&self) -> &str {
        "generic error"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
