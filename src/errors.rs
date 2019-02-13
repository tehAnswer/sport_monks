use std::error::Error;
use std::fmt;

#[derive(Debug, Deserialize)]
struct StandardError {
    message: String,
    code: u32,
}

#[derive(Debug, Deserialize)]
pub struct SportMonksError {
    error: StandardError
}

impl SportMonksError {
    pub fn new(code: u32, message: String) -> SportMonksError {
        let error = StandardError { message, code };
        SportMonksError { error }
    }
}

impl Error for SportMonksError {
    fn description(&self) -> &str {
        self.error.description()
    }

    fn cause(&self) -> Option<&Error> {
        Some(&self.error)
    }
}

impl fmt::Display for SportMonksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl fmt::Display for StandardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for StandardError {
    fn description(&self) -> &str {
        "ERROR!"
    }
}