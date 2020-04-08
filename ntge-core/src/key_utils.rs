use ed25519_dalek::Keypair;
use ed25519_dalek::{SecretKey, PublicKey};
use ed25519_dalek::{ExpandedSecretKey, Signature};
use curve25519_dalek::edwards::{EdwardsPoint, CompressedEdwardsY};
use x25519_dalek::StaticSecret;
use sha2::{Sha512, Digest};
pub use crate::ed25519::*;

use super::error;

pub const CURVE_NAME_ED25519: &str = "Ed25519";

pub fn keypair_validation(private_key: &SecretKey, public_key: &PublicKey) -> Result<bool, error::CoreError> {
    let keypair: Keypair = construct_from_private_key(private_key);
    if  keypair.public.to_bytes() != (*public_key).to_bytes() {
        let core_error = error::CoreError::KeyInvalidError {
            name: CURVE_NAME_ED25519,
            reason: "The given public key and private key do not match.",
        };
        return Err(core_error);
    }

    let expanded_private: ExpandedSecretKey = ExpandedSecretKey::from(private_key);
    let message: &[u8] = b"Test";
    let signature: Signature = expanded_private.sign(message, public_key);

    match public_key.verify(message, &signature) {
        Ok(()) => Ok(true),
        Err(_) => {
            let core_error = error::CoreError::KeyInvalidError {
                name: CURVE_NAME_ED25519,
                reason: "The given public key and private key do not match.",
            };
            return Err(core_error);
        }
    }
}

pub fn ed25519_public_key_to_x25519(public_key: &PublicKey) -> x25519_dalek::PublicKey {
    let public_key_bytes: [u8; 32] = (*public_key).to_bytes();
    let mut bits: [u8; 32] = [0u8; 32];
    bits.copy_from_slice(&public_key_bytes[..32]);
    let compressed = CompressedEdwardsY(bits);
    let edwardspoint: EdwardsPoint = compressed.decompress().unwrap();

    let x25519_public_key: x25519_dalek::PublicKey = edwardspoint.to_montgomery().to_bytes().into();

    x25519_public_key
}

// https://github.com/jedisct1/libsodium/blob/927dfe8e2eaa86160d3ba12a7e3258fbc322909c/src/libsodium/crypto_sign/ed25519/ref10/keypair.c#L69-L83
pub fn ed25519_private_key_to_x25519(private_key: &SecretKey) -> StaticSecret {
    let mut private_key_x25519 = [0; 32];
    private_key_x25519.copy_from_slice(&Sha512::digest(&(private_key.as_bytes())[0..32])[0..32]);

    // https://moderncrypto.org/mail-archive/curves/2014/000293.html
    private_key_x25519[0] &= 248;
    private_key_x25519[31] &= 127;
    private_key_x25519[31] |= 64;

    private_key_x25519.into()
}
