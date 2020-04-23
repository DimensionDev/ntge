// We re-write message encryption logic here from rage by @str4d
// https://github.com/str4d/rage

pub mod filekey;
pub mod private;
pub mod public;

use hkdf::Hkdf;
use rand::rngs::OsRng;
use rand::RngCore;
use secrecy::{ExposeSecret, Secret};
use sha2::Sha256;
use x25519_dalek::{EphemeralSecret, PublicKey, StaticSecret};

use crate::{aead, error::CoreError, message::recipient::MessageRecipientHeader};
