use bech32::{self, FromBase32, ToBase32};
use rand::rngs::OsRng;

use std::marker::Sized;

use super::error;
use super::ed25519;

pub trait Serializable where Self: Sized {
    fn serialize(&self) -> String;
    fn deserialize(encoded: &str) -> Result<Self, error::CoreError>;
}

pub trait KeyType: Serializable {
    // fn generate() -> Self;
}

pub trait PrivateKey: KeyType {
    // fn generate() -> Self;
}

pub trait PublicKey: KeyType {
    // fn generate() -> Self;
}

pub trait KeyPairType {
    type PublicKey: PublicKey;
    type PrivateKey: PrivateKey;
    fn generate() -> Self;

    /// The public key for the key pair.
    fn public_key(&self) -> Self::PublicKey;
    /// The private key for the key pair.
    fn private_key(&self) -> &Self::PrivateKey;
}

pub struct KeyPair;

impl Serializable for ed25519_dalek::SecretKey {
    fn serialize(&self) -> String {
        let data = self.to_bytes().to_base32();
        let encoded = bech32::encode("pri", data).unwrap();
        encoded + "-" + "CURVE_NAME_ED25519"
    }

    fn deserialize(encoded: &str) -> Result<Self, error::CoreError> {
        ed25519::deserialize_private_key(encoded)
    }
}

impl KeyType for ed25519_dalek::SecretKey {}
impl PrivateKey for ed25519_dalek::SecretKey {}

impl Serializable for ed25519_dalek::PublicKey {
    fn serialize(&self) -> String {
        let data = self.to_bytes().to_base32();
        let encoded = bech32::encode("pub", data).unwrap();
        encoded + "-" + "CURVE_NAME_ED25519"
    }

    fn deserialize(encoded: &str) -> Result<Self, error::CoreError> {
        ed25519::deserialize_public_key(encoded)
    }
}

impl KeyType for ed25519_dalek::PublicKey {}
impl PublicKey for ed25519_dalek::PublicKey {}

impl KeyPairType for ed25519_dalek::Keypair {
    type PublicKey = ed25519_dalek::PublicKey;
    type PrivateKey = ed25519_dalek::SecretKey;
    fn generate() -> Self {
        let mut csprng: OsRng = OsRng {};
        ed25519_dalek::Keypair::generate(&mut csprng)
    }

    /// The public key for the key pair.
    fn public_key(&self) -> Self::PublicKey {
        self.public
    }
    /// The private key for the key pair.
    fn private_key(&self) -> &Self::PrivateKey {
        &self.secret
    }
}

pub enum KeyAlgo {
    Ed25519
}

pub fn generate_keypair<P: KeyPairType>(algo: KeyAlgo) -> P {
    match algo {
        KeyAlgo::Ed25519 => {
            P::generate()
        }
    }
}