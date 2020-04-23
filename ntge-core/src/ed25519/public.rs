use bech32::{self, FromBase32, ToBase32};
use ed25519_dalek::{self, PublicKey};

use crate::strings;
use std::os::raw::c_char;

use super::{error, private::Ed25519PrivateKey, CURVE_NAME_ED25519};

#[derive(Debug)]
pub struct Ed25519PublicKey {
    pub raw: ed25519_dalek::PublicKey,
}

impl Ed25519PublicKey {
    pub fn serialize(&self) -> String {
        let data = self.raw.to_bytes().to_base32();
        let encoded = bech32::encode("pub", data).unwrap();
        encoded + "-" + CURVE_NAME_ED25519
    }

    pub fn deserialize(encoded: &str) -> Result<Ed25519PublicKey, error::CoreError> {
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

            Ok(Ed25519PublicKey { raw: public_key })
        }
    }
}

impl<'a> From<&'a Ed25519PrivateKey> for Ed25519PublicKey {
    fn from(private_key: &Ed25519PrivateKey) -> Ed25519PublicKey {
        Ed25519PublicKey {
            raw: (&private_key.raw).into(),
        }
    }
}

impl Clone for Ed25519PublicKey {
    fn clone(&self) -> Self {
        Ed25519PublicKey {
            raw: self.raw.clone(),
        }
    }
}

impl Drop for Ed25519PublicKey {
    fn drop(&mut self) {
        println!("{:?} is being deallocated", self);
    }
}

#[no_mangle]
pub extern "C" fn c_ed25519_public_key_destroy(public_key: &mut Ed25519PublicKey) {
    let _ = unsafe { Box::from_raw(public_key) };
}

#[no_mangle]
pub unsafe extern "C" fn c_ed25519_public_key_serialize(
    public_key: *mut Ed25519PublicKey,
) -> *mut c_char {
    let public_key = &mut *public_key;
    strings::string_to_c_char(public_key.serialize())
}

#[no_mangle]
pub unsafe extern "C" fn c_ed25519_public_key_deserialize(
    encoded: *const c_char,
) -> *mut Ed25519PublicKey {
    let encoded = strings::c_char_to_string(encoded);
    let public_key = Ed25519PublicKey::deserialize(&encoded);
    match public_key {
        Ok(key) => Box::into_raw(Box::new(key)),
        Err(_) => std::ptr::null_mut(),
    }
}
