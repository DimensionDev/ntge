use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct SignatureError {
    pub(crate) message: String,
}

impl fmt::Display for SignatureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for SignatureError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
