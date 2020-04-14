pub use ed25519_dalek::{self, PublicKey, SecretKey};
use rand::rngs::OsRng;

use crate::ed25519::private::Ed25519PrivateKey;

#[derive(Debug)]
pub struct Ed25519Keypair {
    pub raw: ed25519_dalek::Keypair,
}

impl Ed25519Keypair {
    pub fn new() -> Self {
        // a.k.a Cryptographically secure pseudo-random number generator.
        let mut csprng = OsRng {};
        Ed25519Keypair {
            raw: ed25519_dalek::Keypair::generate(&mut csprng),
        }
    }

    pub fn construct_from_private_key(private_key: &Ed25519PrivateKey) -> Self {
        let sk: SecretKey = SecretKey::from_bytes(&(private_key.raw.to_bytes())).unwrap();
        let pk: PublicKey = (&sk).into();

        let keypair = ed25519_dalek::Keypair {
            secret: sk,
            public: pk,
        };

        Ed25519Keypair { raw: keypair }
    }
}

impl Drop for Ed25519Keypair {
    fn drop(&mut self) {
        println!("{:?} is being deallocated", self);
    }
}
