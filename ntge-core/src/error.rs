use core::fmt;
use core::fmt::Display;

#[cfg(feature = "std")]
use std::error::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CoreError {
    KeyDeserializeError {
        name: &'static str,
        reason: &'static str,
    },
    MessageDecryptionError {
        name: &'static str,
        reason: &'static str,
    },
    MessageSerializationError {
        name: &'static str,
        reason: &'static str,
    },
    KeyInvalidError {
        name: &'static str,
        reason: &'static str,
    },
    SignatureVerificationError {
        name: &'static str,
        reason: &'static str,
    },
}

#[cfg(feature = "std")]
impl Error for CoreError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self)
    }
}

impl Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CoreError::KeyDeserializeError { name, reason } => {
                write!(f, "deserialize {} - {}", name, reason)
            }
            CoreError::MessageDecryptionError { name, reason } => {
                write!(f, "decryption {} - {}", name, reason)
            }
            CoreError::MessageSerializationError { name, reason } => {
                write!(f, "serialize {} - {}", name, reason)
            }
            CoreError::KeyInvalidError { name, reason } => {
                write!(f, "is not a valid {} keypair. {}", name, reason)
            }
            CoreError::SignatureVerificationError { name, reason } => {
                write!(f, "The {} is not verified. {}", name, reason)
            }
        }
    }
}
