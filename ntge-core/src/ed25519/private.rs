use bech32::{self, FromBase32, ToBase32};
use ed25519_dalek::{self, PublicKey, SecretKey};
use ed25519_dalek::{ExpandedSecretKey, Signature};
use rand::rngs::OsRng;

use crate::strings;
use std::os::raw::c_char;

use crate::{
    ed25519::keypair::Ed25519Keypair, ed25519::public::Ed25519PublicKey,
    ed25519::CURVE_NAME_ED25519, error,
};

#[derive(Debug)]
pub struct Ed25519PrivateKey {
    pub raw: ed25519_dalek::SecretKey,
}

impl Ed25519PrivateKey {
    pub fn get_public_key(&self) -> Ed25519PublicKey {
        let keypair = Ed25519Keypair::construct_from_private_key(&self);
        keypair.get_public_key()
    }
}

impl Ed25519PrivateKey {
    pub fn new() -> Self {
        let mut csprng = OsRng {};
        Ed25519PrivateKey {
            raw: SecretKey::generate(&mut csprng),
        }
    }

    /// Serialize private key to string represent
    pub fn serialize(&self) -> String {
        let data = self.raw.to_bytes().to_base32();
        let encoded = bech32::encode("pri", data).unwrap();
        encoded + "-" + CURVE_NAME_ED25519
    }

    pub fn deserialize(encoded: &str) -> Result<Self, error::CoreError> {
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

            Ok(Ed25519PrivateKey { raw: private_key })
        }
    }
}

impl Drop for Ed25519PrivateKey {
    fn drop(&mut self) {
        if cfg!(feature = "drop-log-enable") {
            println!("{:?} is being deallocated", self);
        }
    }
}

impl Clone for Ed25519PrivateKey {
    fn clone(&self) -> Self {
        let bytes = self.raw.as_bytes().clone();
        Ed25519PrivateKey {
            raw: ed25519_dalek::SecretKey::from_bytes(&bytes)
                .expect("should generate a new raw key from its bytes"),
        }
    }
}

pub fn serialize_private_key(private_key: &SecretKey) -> String {
    let data = private_key.to_bytes().to_base32();
    let encoded = bech32::encode("pri", data).unwrap();
    encoded + "-" + CURVE_NAME_ED25519
}

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

pub fn sign(private_key: &SecretKey, message: &[u8]) -> Signature {
    let expanded_private_key: ExpandedSecretKey = private_key.into();
    let public_key: PublicKey = private_key.into();

    let signature: Signature = expanded_private_key.sign(message, &public_key);
    signature
}

#[no_mangle]
pub extern "C" fn c_ed25519_private_key_new() -> *mut Ed25519PrivateKey {
    let private_key = Ed25519PrivateKey::new();
    Box::into_raw(Box::new(private_key))
}

#[no_mangle]
pub extern "C" fn c_ed25519_private_key_destroy(private_key: &mut Ed25519PrivateKey) {
    let _ = unsafe { Box::from_raw(private_key) };
}

#[no_mangle]
pub unsafe extern "C" fn c_ed25519_private_key_get_public_key(
    private_key: *mut Ed25519PrivateKey,
) -> *mut Ed25519PublicKey {
    let private_key = &mut *private_key;
    let public_key = private_key.get_public_key();
    Box::into_raw(Box::new(public_key))
}

#[no_mangle]
pub unsafe extern "C" fn c_ed25519_private_key_serialize(
    private_key: *mut Ed25519PrivateKey,
) -> *mut c_char {
    let private_key = &mut *private_key;
    strings::string_to_c_char(private_key.serialize())
}

#[no_mangle]
pub unsafe extern "C" fn c_ed25519_private_key_deserialize(
    encoded: *const c_char,
) -> *mut Ed25519PrivateKey {
    let encoded = strings::c_char_to_string(encoded);
    let private_key = Ed25519PrivateKey::deserialize(&encoded);
    match private_key {
        Ok(key) => Box::into_raw(Box::new(key)),
        Err(_) => std::ptr::null_mut(),
    }
}

#[cfg(test)]
mod tests {
    use crate::ed25519;

    #[test]
    fn it_deserializes_a_private_key() {
        let encoded_private_key =
            "pri1kq9sn9nyutfwsrauz2akl0d0qxzu38dnes6q47x6tnaf57ad7xnsg2fq6l-Ed25519";
        let deserialized_private_key =
            ed25519::private::deserialize_private_key(&encoded_private_key);
        assert_eq!(true, deserialized_private_key.is_ok());
    }
}
