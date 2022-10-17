use std;
use std::fmt;

pub mod postgres;

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

pub enum SourceError {
    CommandError(Error),
    DumpError(Error),
}
