use core::fmt;
use core::fmt::Display;

use ed25519_dalek::SignatureError;
use bson::EncoderError;
use bson::DecoderError;

#[cfg(feature = "std")]
use std::error::Error;

#[derive(Debug)]
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
                write!(f, "cannot deserialize {}: {}", name, reason)
            }
            CoreError::MessageDecryptionError { name, reason } => {
                write!(f, "cannot decryption {}: {}", name, reason)
            }
            CoreError::MessageSerializationError { name, reason } => {
                write!(f, "cannot serialize {}: {}", name, reason)
            }
            CoreError::KeyInvalidError { name, reason } => {
                write!(f, "is not a valid {} keypair. {}", name, reason)
            }
        }
    }
}

impl From<SignatureError> for CoreError {
    fn from(_: SignatureError) -> Self {
        CoreError::KeyDeserializeError {
            name: "PublicKey",
            reason: "cannot restore key from payload",
        }
    }
}

impl From<bech32::Error> for CoreError {
    fn from(_: bech32::Error) -> Self {
        CoreError::KeyDeserializeError {
            name: "PrivateKey",
            reason: "cannot decode base32 key payload",
        }
    }
}

impl From<EncoderError> for CoreError {
    fn from(_: EncoderError) -> Self {
        CoreError::MessageSerializationError {
            name: "Message",
            reason: "cannot encode message to bson",
        }
    }
}

impl From<DecoderError> for CoreError {
    fn from(_: DecoderError) -> Self {
        CoreError::KeyDeserializeError {
            name: "Message",
            reason: "cannot decode bson bytes to message document",
        }
    }
}