use bech32::{self, FromBase32, ToBase32};
use ed25519_dalek::Keypair;
use ed25519_dalek::{SecretKey, PublicKey};
use ed25519_dalek::{ExpandedSecretKey, Signature};
use rand::rngs::OsRng;
pub use crate::ed25519::*;

use super::error;

pub const CURVE_NAME_ED25519: &'static str = "Ed25519";

pub fn keypair_validation(private_key: &SecretKey, public_key: &PublicKey) -> bool {
    let keypair: Keypair = from_private_key(private_key);
    if  keypair.public.to_bytes() != (*public_key).to_bytes() {
        return false;
    }

    let expanded_private: ExpandedSecretKey = ExpandedSecretKey::from(private_key);
    let message: &[u8] = "Test".as_bytes();
    let signature: Signature = expanded_private.sign(message, public_key);

    match public_key.verify(message, &signature) {
        Ok(()) => true,
        Err(err) => false
    }
}
