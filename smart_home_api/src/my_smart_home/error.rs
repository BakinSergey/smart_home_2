use std::fmt::{Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct SmartHomeError {
    pub msg: String,
}

impl std::fmt::Display for SmartHomeError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for SmartHomeError {
    fn description(&self) -> &str {
        &self.msg
    }
}

impl From<String> for SmartHomeError {
    fn from(err: String) -> Self {
        SmartHomeError { msg: err }
    }
}

impl From<&str> for SmartHomeError {
    fn from(err: &str) -> Self {
        SmartHomeError {
            msg: err.to_string(),
        }
    }
}

pub type SmartHomeResult<T> = Result<T, SmartHomeError>;
