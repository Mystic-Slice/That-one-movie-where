use std::{fmt, error::Error};

#[derive(Debug, Clone)]
pub struct MovieError {
    details: String
}

impl MovieError {
    pub fn new(msg: &str) -> MovieError {
        MovieError{details: msg.to_string()}
    }
}

impl fmt::Display for MovieError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for MovieError {
    fn description(&self) -> &str {
        &self.details
    }
}