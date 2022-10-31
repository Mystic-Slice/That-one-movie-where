use std::fmt;

#[derive(Debug, Clone)]
/// Type for custom errors
/// TODO: Probably convert to enum and have more descriptive error messages
pub struct MovieError {
    details: String
}

impl MovieError {
    pub fn new(msg: &str) -> MovieError {
        MovieError{details: msg.to_string()}
    }
}

/// To enable printing of errors
impl fmt::Display for MovieError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}