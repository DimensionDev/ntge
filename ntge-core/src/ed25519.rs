use bech32::{self, FromBase32, ToBase32};
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;

pub use ed25519_dalek::{self, PublicKey, SecretKey};

pub mod keypair;
pub mod private;
pub mod public;

use crate::ed25519::{
    keypair::Ed25519Keypair, private::Ed25519PrivateKey, public::Ed25519PublicKey,
};
use crate::error;

pub const CURVE_NAME_ED25519: &str = "Ed25519";

#[deprecated]
pub fn create_keypair() -> Keypair {
    // a.k.a Cryptographically secure pseudo-random number generator.
    let mut csprng: OsRng = OsRng {};
    ed25519_dalek::Keypair::generate(&mut csprng)
}

#[deprecated]
pub fn construct_from_private_key(private_key: &SecretKey) -> Keypair {
    let sk: SecretKey = SecretKey::from_bytes(&(private_key.to_bytes())).unwrap();
    let pk: PublicKey = (&sk).into();

    Keypair {
        public: pk,
        secret: sk,
    }
}

#[deprecated]
pub fn serialize_private_key(private_key: &SecretKey) -> String {
    let data = private_key.to_bytes().to_base32();
    let encoded = bech32::encode("pri", data).unwrap();
    encoded + "-" + CURVE_NAME_ED25519
}

#[deprecated]
pub fn serialize_public_key(public_key: &PublicKey) -> String {
    let data = public_key.to_bytes().to_base32();
    let encoded = bech32::encode("pub", data).unwrap();
    encoded + "-" + CURVE_NAME_ED25519
}

#[deprecated]
pub fn deserialize_private_key(encoded: &str) -> Result<SecretKey, error::CoreError> {
    let components: Vec<&str> = encoded.trim().split('-').collect();
    if components.len() != 2 {
        // return Err if encoded text not have two components like:
        // <bech32>-<curve_name>
        let e = error::CoreError::KeyDeserializeError {
            name: "PrivateKey",
            reason: "cannot parse key from text",
        };
        Err(e)
    } else {
        let curve_name = components[1];

        // 1. check curve name
        if curve_name != CURVE_NAME_ED25519 {
            let e = error::CoreError::KeyDeserializeError {
                name: "PrivateKey",
                reason: "cannot read key curve name",
            };
            return Err(e);
        }

        // 2. decode bech32 to base32
        let bech32_encoded = components[0];
        let (hrp, base32_encoded) = match bech32::decode(&bech32_encoded) {
            Ok(tuple) => tuple,
            Err(_) => {
                let core_error = error::CoreError::KeyDeserializeError {
                    name: "PrivateKey",
                    reason: "cannot decode bech32 key payload",
                };
                return Err(core_error);
            }
        };

        // 3. check hrp
        if hrp != "pri" {
            let e = error::CoreError::KeyDeserializeError {
                name: "PrivateKey",
                reason: "cannot read invalid key payload",
            };
            return Err(e);
        }

        // 4. decode base32 to bytes
        let bytes = match Vec::<u8>::from_base32(&base32_encoded) {
            Ok(bytes) => bytes,
            Err(_) => {
                let e = error::CoreError::KeyDeserializeError {
                    name: "PrivateKey",
                    reason: "cannot decode base32 key payload",
                };
                return Err(e);
            }
        };

        // 5. restore key from bytes
        let private_key = match SecretKey::from_bytes(&bytes) {
            Ok(key) => key,
            Err(_) => {
                let e = error::CoreError::KeyDeserializeError {
                    name: "PrivateKey",
                    reason: "cannot restore key from payload",
                };
                return Err(e);
            }
        };

        Ok(private_key)
    }
}

pub fn deserialize_public_key(encoded: &str) -> Result<PublicKey, error::CoreError> {
    let components: Vec<&str> = encoded.trim().split('-').collect();
    if components.len() != 2 {
        // return Err if encoded text not have two components like:
        // <bech32>-<curve_name>
        let e = error::CoreError::KeyDeserializeError {
            name: "PublicKey",
            reason: "cannot parse key from text",
        };
        Err(e)
    } else {
        let curve_name = components[1];

        // 1. check curve name
        if curve_name != CURVE_NAME_ED25519 {
            let e = error::CoreError::KeyDeserializeError {
                name: "PublicKey",
                reason: "cannot read key curve name",
            };
            return Err(e);
        }

        // 2. decode bech32 to base32
        let bech32_encoded = components[0];
        let (hrp, base32_encoded) = match bech32::decode(&bech32_encoded) {
            Ok(tuple) => tuple,
            Err(_) => {
                let core_error = error::CoreError::KeyDeserializeError {
                    name: "PublicKey",
                    reason: "cannot decode bech32 key payload",
                };
                return Err(core_error);
            }
        };

        // 3. check hrp
        if hrp != "pub" {
            let e = error::CoreError::KeyDeserializeError {
                name: "PublicKey",
                reason: "invalid public key payload",
            };
            return Err(e);
        }

        // 4. decode base32 to bytes
        let bytes = match Vec::<u8>::from_base32(&base32_encoded) {
            Ok(bytes) => bytes,
            Err(_) => {
                let e = error::CoreError::KeyDeserializeError {
                    name: "PublicKey",
                    reason: "cannot decode base32 key payload",
                };
                return Err(e);
            }
        };

        // 5. restore key from bytes
        let public_key = match PublicKey::from_bytes(&bytes) {
            Ok(key) => key,
            Err(_) => {
                let e = error::CoreError::KeyDeserializeError {
                    name: "PublicKey",
                    reason: "cannot restore key from payload",
                };
                return Err(e);
            }
        };

        Ok(public_key)
    }
}

#[no_mangle]
pub extern "C" fn c_ed25519_keypair_new() -> *mut Ed25519Keypair {
    let keypair = Ed25519Keypair::new();
    Box::into_raw(Box::new(keypair))
}

#[no_mangle]
pub extern "C" fn c_ed25519_keypair_destroy(keypair: *mut Ed25519Keypair) {
    let _ = unsafe { Box::from_raw(keypair) };
}

#[no_mangle]
pub extern "C" fn c_ed25519_keypair_construct_from_private_key(
    private_key: &Ed25519PrivateKey,
) -> *mut Ed25519Keypair {
    let keypair = Ed25519Keypair::construct_from_private_key(&private_key);
    Box::into_raw(Box::new(keypair))
}
