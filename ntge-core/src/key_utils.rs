use ed25519_dalek::Keypair;
use ed25519_dalek::{SecretKey, PublicKey};
use ed25519_dalek::{ExpandedSecretKey, Signature};
use curve25519_dalek::edwards::{EdwardsPoint, CompressedEdwardsY};
use x25519_dalek;
pub use crate::ed25519::*;

pub const CURVE_NAME_ED25519: &str = "Ed25519";

pub fn keypair_validation(private_key: &SecretKey, public_key: &PublicKey) -> bool {
    let keypair: Keypair = construct_from_private_key(private_key);
    if  keypair.public.to_bytes() != (*public_key).to_bytes() {
        return false;
    }

    let expanded_private: ExpandedSecretKey = ExpandedSecretKey::from(private_key);
    let message: &[u8] = b"Test";
    let signature: Signature = expanded_private.sign(message, public_key);

    match public_key.verify(message, &signature) {
        Ok(()) => true,
        Err(_) => false
    }
}

pub fn ed25519_to_x25519(public_key: &PublicKey) -> x25519_dalek::PublicKey {
    println!("{:?}", public_key);

    let public_key_bytes: [u8; 32] = (*public_key).to_bytes();
    let mut bits: [u8; 32] = [0u8; 32];
    bits.copy_from_slice(&public_key_bytes[..32]);
    let compressed = CompressedEdwardsY(bits);
    let edwardspoint: EdwardsPoint = compressed.decompress().unwrap();

    let x25519_public_key: x25519_dalek::PublicKey = edwardspoint.to_montgomery().to_bytes().into();

    x25519_public_key
}
