use bech32::{self, FromBase32, ToBase32};
use ed25519_dalek::Signature;
use ed25519_dalek::{self, PublicKey};
use sha3::{Digest, Sha3_256};

use super::{error, private::Ed25519PrivateKey, CURVE_NAME_ED25519};

#[cfg(target_os = "ios")]
use crate::strings;

#[cfg(target_os = "ios")]
use std::os::raw::c_char;

#[cfg(target_os = "ios")]
use crate::buffer::Buffer;

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

impl Ed25519PublicKey {
    pub fn key_id(&self) -> String {
        let data = self.raw.to_bytes();
        let base64_encoded_bytes: Vec<u8> = base64::encode(data).bytes().collect();
        // use sha3-256 produce keyID from base64_encoded_bytes
        let mut hasher = Sha3_256::new();
        hasher.input(base64_encoded_bytes);
        let result = hasher.result();
        hex::encode(result)
    }
}

impl Ed25519PublicKey {
    pub fn verify(&self, message: &[u8], signature_bytes: &[u8]) -> Result<(), error::CoreError> {
        let signature: Signature = match Signature::from_bytes(&signature_bytes) {
            Ok(signature) => signature,
            Err(_) => {
                let e = error::CoreError::SignatureVerificationError {
                    name: "signature",
                    reason: "The signature data is not valid.",
                };
                return Err(e);
            }
        };

        match self.raw.verify(message, &signature) {
            Ok(_) => Ok(()),
            Err(_) => {
                let e = error::CoreError::SignatureVerificationError {
                    name: "signature",
                    reason: "The signature does not match the given public key.",
                };
                Err(e)
            }
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
        Ed25519PublicKey { raw: self.raw }
    }
}

impl Drop for Ed25519PublicKey {
    fn drop(&mut self) {
        if cfg!(feature = "drop-log-enable") {
            println!("{:?} is being deallocated", self);
        }
    }
}

#[no_mangle]
#[cfg(target_os = "ios")]
pub extern "C" fn c_ed25519_public_key_destroy(public_key: &mut Ed25519PublicKey) {
    let _ = unsafe { Box::from_raw(public_key) };
}

#[no_mangle]
#[cfg(target_os = "ios")]
pub unsafe extern "C" fn c_ed25519_public_key_serialize(
    public_key: *mut Ed25519PublicKey,
) -> *mut c_char {
    let public_key = &mut *public_key;
    strings::string_to_c_char(public_key.serialize())
}

#[no_mangle]
#[cfg(target_os = "ios")]
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

#[no_mangle]
#[cfg(target_os = "ios")]
pub unsafe extern "C" fn c_ed25519_public_key_key_id(
    public_key: *mut Ed25519PublicKey,
) -> *mut c_char {
    let public_key = &mut *public_key;
    let key_id = public_key.key_id();
    strings::string_to_c_char(key_id)
}

#[no_mangle]
#[cfg(target_os = "ios")]
pub unsafe extern "C" fn c_ed25519_public_key_verify(
    public_key: *mut Ed25519PublicKey,
    message_buffer: Buffer,
    signature_buffer: Buffer,
) -> i32 {
    let public_key = &mut *public_key;
    let message_bytes = message_buffer.to_bytes();
    let signature_bytes = signature_buffer.to_bytes();

    // verify signature
    match public_key.verify(&message_bytes, &signature_bytes) {
        Ok(_) => 0,
        Err(_) => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::Ed25519PublicKey;
    use crate::ed25519;
    use sha3::{Digest, Sha3_256};

    #[test]
    fn it_deserializes_a_public_key() {
        let encoded_public_key =
            "pub1w0yqh0eple0cpeqc0es7um553e6pfmyuam6x6cu3vaq602d7v6msnal7n5-Ed25519";
        let deserialized_public_key =
            ed25519::public::Ed25519PublicKey::deserialize(&encoded_public_key);
        assert_eq!(true, deserialized_public_key.is_ok());
    }

    #[test]
    fn it_signs_a_message_and_self_verify() {
        let message = "TEST";
        let keypair = ed25519::keypair::Ed25519Keypair::new();

        let signature = &keypair.get_private_key().sign(message.as_bytes());
        let result = &keypair
            .get_public_key()
            .verify(message.as_bytes(), &signature[..]);
        assert!(result.is_ok());
    }

    #[test]
    fn it_generates_a_message_key_id() {
        let keypair = ed25519::keypair::Ed25519Keypair::new();
        let public_key = keypair.get_public_key();
        let key_id = public_key.key_id();
        let key_id_2 = public_key.key_id();
        println!("{}", key_id);
        assert!(key_id.len() > 0);
        assert_eq!(key_id, key_id_2);

        // test stub
        let stub_public_key = Ed25519PublicKey {
            raw: ed25519_dalek::PublicKey::from_bytes(&[0; 32]).unwrap(),
        };
        let stub_key_id = stub_public_key.key_id();

        let base64_encoded_32_zeros_bytes: Vec<u8> = base64::encode([0; 32]).bytes().collect();
        let mut hasher = Sha3_256::new();
        hasher.input(base64_encoded_32_zeros_bytes);
        let result = hasher.result();
        let calculated_stub_key_id = hex::encode(result);
        println!("{}", stub_key_id);
        println!("{}", calculated_stub_key_id);
        assert_eq!(stub_key_id, calculated_stub_key_id);
    }
}
