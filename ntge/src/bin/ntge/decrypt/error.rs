use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct DecryptError {
    pub(crate) message: String,
}

impl fmt::Display for DecryptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for DecryptError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
