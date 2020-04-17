use ed25519_dalek::Signature;
use ed25519_dalek::{PublicKey, SecretKey};

use crate::ed25519;

#[derive(Debug)]
pub struct Signer {
    pub message: Vec<u8>,
}

impl Signer {
    pub fn new(message: Vec<u8>) -> Self {
        Signer { message }
    }
}

#[allow(dead_code)]
impl Signer {
    pub fn sign(&self, private_key: &SecretKey) -> [u8; 64] {
        let signature: Signature = ed25519::sign(private_key, &self.message);

        signature.to_bytes()
    }

    pub fn verify(
        &self,
        public_key: &PublicKey,
        signature_bytes: &[u8; 64],
    ) -> Result<bool, &'static str> {
        let signature = match Signature::from_bytes(signature_bytes) {
            Ok(sig) => sig,
            Err(_) => return Err("Error parsing the signature"),
        };
        match ed25519::verify(public_key, &self.message, &signature) {
            Ok(r) => Ok(r),
            Err(_) => Err("Error verifying the signature"),
        }
    }
}
