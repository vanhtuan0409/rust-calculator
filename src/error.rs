use std::error;
use std::fmt;

#[derive(Debug)]
pub struct ExitError {
    pub code: i32,
    pub message: String,
}

impl fmt::Display for ExitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for ExitError {}

pub type Result<T> = ::std::result::Result<T, ExitError>;

#[allow(dead_code)]
pub fn exit(code: i32, message: String) -> Result<()> {
    Err(ExitError { code, message })
}
