use bs58;
use bson;
use serde::{Deserialize, Serialize};
use serde_bytes;
use x25519_dalek::PublicKey;

use crate::error::CoreError;

pub mod decryptor;
pub mod encryptor;

pub(crate) const MAC_KEY_LABEL: &[u8] = b"ntge-message-mac-key";
pub(crate) const PAYLOAD_KEY_LABEL: &[u8] = b"ntge-message-payload";

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageRecipientHeader {
    pub key_type: String,
    #[serde(with = "serde_bytes")]
    pub ephemeral_public_key: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub encrypted_file_key: Vec<u8>, // 32 bytes
}

impl MessageRecipientHeader {
    pub fn get_ephemeral_public_key(&self) -> PublicKey {
        let mut key_bytes = [0; 32];
        key_bytes.copy_from_slice(&self.ephemeral_public_key[..32]);
        PublicKey::from(key_bytes)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageMeta {
    pub timestamp: Option<String>,
    pub signature_a: [u8; 32],
    pub signature_b: [u8; 32],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageMac {
    #[serde(with = "serde_bytes")]
    pub mac: Vec<u8>, // 32 bytes
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessagePayload {
    #[serde(with = "serde_bytes")]
    pub nonce: Vec<u8>, // 16 bytes
    #[serde(with = "serde_bytes")]
    pub ciphertext: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub recipient_headers: Vec<MessageRecipientHeader>,
    pub meta: MessageMeta,
    pub mac: MessageMac,
    pub payload: MessagePayload,
}

#[allow(dead_code)]
impl Message {
    pub fn serialize_to_armor(&self) -> Result<String, CoreError> {
        match self.serialize_to_base58() {
            Ok(base58) => Ok(format!("MsgBegin_{}_EndMsg", base58)),
            Err(e) => Err(e),
        }
    }

    pub fn deserialize_from_armor(text: &str) -> Result<Message, CoreError> {
        let text = text.trim();
        let has_prefix = text.starts_with("MsgBegin_");
        let has_suffix = text.ends_with("_EndMsg");

        if has_prefix && has_suffix {
            let text = text.trim_start_matches("MsgBegin_");
            let text = text.trim_end_matches("_EndMsg");
            Message::deserialize_from_base58(text)
        } else {
            // no armor
            let e = CoreError::MessageSerializationError {
                name: "Message",
                reason: "cannot deserialize message armor text",
            };
            Err(e)
        }
    }
}

impl Message {
    pub fn serialize_to_base58(&self) -> Result<String, CoreError> {
        self.serialize_to_bson_bytes()
            .map(|bytes| bs58::encode(bytes).into_string())
            .map_err(|_| CoreError::MessageSerializationError {
                name: "Message",
                reason: "cannot encode message bytes to base58",
            })
    }

    pub fn deserialize_from_base58(text: &str) -> Result<Message, CoreError> {
        let bytes = match bs58::decode(text).into_vec() {
            Ok(bytes) => bytes,
            Err(_) => {
                let e = CoreError::MessageSerializationError {
                    name: "Message",
                    reason: "cannot decode message base58 text",
                };
                return Err(e);
            }
        };

        Message::deserialize_from_bson_bytes(&bytes)
    }
}

#[allow(dead_code)]
impl Message {
    fn serialize_to_bson_bytes(&self) -> Result<Vec<u8>, CoreError> {
        let document = match bson::to_bson(&self) {
            Ok(encoded) => {
                if let bson::Bson::Document(document) = encoded {
                    document
                } else {
                    let e = CoreError::MessageSerializationError {
                        name: "Message",
                        reason: "cannot encode message to bson",
                    };
                    return Err(e);
                }
            }
            Err(err) => {
                print!("{:?}", err);
                let e = CoreError::MessageSerializationError {
                    name: "Message",
                    reason: "cannot encode message to bson",
                };
                return Err(e);
            }
        };

        let mut bytes = Vec::new();
        match bson::encode_document(&mut bytes, &document) {
            Ok(_) => Ok(bytes),
            Err(_) => {
                let e = CoreError::KeyDeserializeError {
                    name: "Message",
                    reason: "cannot encode message bson document to bytes",
                };
                Err(e)
            }
        }
    }

    #[allow(ptr_arg)]
    fn deserialize_from_bson_bytes(bytes: &Vec<u8>) -> Result<Message, CoreError> {
        let document = match bson::decode_document(&mut std::io::Cursor::new(&bytes[..])) {
            Ok(document) => document,
            Err(_) => {
                let e = CoreError::KeyDeserializeError {
                    name: "Message",
                    reason: "cannot decode bson bytes to message document",
                };
                return Err(e);
            }
        };

        let result: Result<Message, _> = bson::from_bson(bson::Bson::Document(document));
        result.map_err(|_| CoreError::KeyDeserializeError {
            name: "Message",
            reason: "cannot decode message document to message",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::{decryptor, encryptor, Message};
    use crate::{aead, x25519::FileKey};
    use crate::{ed25519, key_utils};
    use hkdf::Hkdf;
    use rand::rngs::OsRng;
    use rand::RngCore;
    use secrecy::ExposeSecret;
    use sha2::Sha256;
    use x25519_dalek::{PublicKey, StaticSecret};
    #[allow(dead_code)]
    fn encrypt_plaintext(file_key: &FileKey, plaintext: &[u8]) -> Vec<u8> {
        // prepare nonce
        let mut nonce = [0; 16];
        OsRng.fill_bytes(&mut nonce);
        // create payload key
        let mut payload_key = [0; 32];
        Hkdf::<Sha256>::new(Some(&nonce), file_key.0.expose_secret())
            .expand(PAYLOAD_KEY_LABEL, &mut payload_key)
            .expect("payload_key is the correct length");
        aead::aead_encrypt(&payload_key, &plaintext)
    }

    #[test]
    fn it_encrypts_a_message_to_alice() {
        // plaintext
        let plaintext = b"Hello, World!";
        // create new file key
        let file_key = FileKey::new();
        // pass file key to encrypt plaintext
        let ciphertext = encrypt_plaintext(&file_key, plaintext);
        print!("{:?}", ciphertext);
    }
    #[test]
    fn it_encrypts_and_decrypts_a_message_to_alice() {
        let plaintext = b"Hello, World!";
        // alice
        let mut alice_csprng = OsRng {};
        let alice_secret_key = StaticSecret::new(&mut alice_csprng);
        let alice_public_key = PublicKey::from(&alice_secret_key);

        let encryptor = encryptor::Encryptor::new(vec![alice_public_key]);
        let message = encryptor.encrypt(plaintext, None);

        // create decryptor
        let decryptor = decryptor::Decryptor::new(&message);
        // get file key
        let file_key = decryptor
            .decrypt_file_key(&alice_secret_key)
            .expect("could decrypt file key");
        // check mac
        assert_eq!(decryptor.verify_message_mac(&file_key), true);
        // decrypt message payload
        let decrypted_plaintext = decryptor
            .decrypt_payload(&file_key)
            .expect("could decrypt payload");
        assert_eq!(decrypted_plaintext, plaintext);
    }

    #[test]
    fn it_encodes_and_decodes_a_message_to_alice() {
        let plaintext = b"Hello, World!";

        // alice
        let mut alice_csprng = OsRng {};
        let alice_secret_key = StaticSecret::new(&mut alice_csprng);
        let alice_public_key = PublicKey::from(&alice_secret_key);

        let encryptor = encryptor::Encryptor::new(vec![alice_public_key]);
        let message = encryptor.encrypt(plaintext, None);
        // encode
        let encoded_message = message.serialize_to_armor().expect("could serialize");
        print!("{:?}", encoded_message);
        let decoded_message =
            Message::deserialize_from_armor(&encoded_message).expect("could deserialize");
        assert_eq!(decoded_message.mac.mac, message.mac.mac);
        assert_eq!(decoded_message.meta.timestamp, message.meta.timestamp);
    }

    #[test]
    fn it_encrypts_and_decrypts_a_message_to_alice_and_signs() {
        let plaintext = b"Hello, World!";
        // alice
        let mut alice_csprng = OsRng {};
        let alice_keypair = ed25519::create_keypair();
        let alice_secret_key = alice_keypair.secret;
        let alice_secret_key_x25519 = key_utils::ed25519_private_key_to_x25519(&alice_secret_key);
        let alice_public_key = alice_keypair.public;
        let alice_public_key_x25519 = PublicKey::from(&alice_secret_key_x25519);

        let encryptor = encryptor::Encryptor::new(vec![alice_public_key_x25519]);
        let message = encryptor.encrypt(plaintext, Some(&alice_secret_key));

        // create decryptor
        let decryptor = decryptor::Decryptor::new(&message);
        // get file key
        let file_key = decryptor
            .decrypt_file_key(&alice_secret_key_x25519)
            .expect("could decrypt file key");
        // check mac
        assert_eq!(decryptor.verify_message_mac(&file_key), true);
        // decrypt message payload
        let decrypted_plaintext = decryptor
            .decrypt_payload(&file_key)
            .expect("could decrypt payload");
        assert_eq!(decrypted_plaintext, plaintext);
    }

    #[test]
    fn it_encrypts_and_decrypts_a_message_to_alice_and_signs_and_verifies() {
        let plaintext = b"Hello, World!";
        // alice
        let mut alice_csprng = OsRng {};
        let alice_keypair = ed25519::create_keypair();
        let alice_secret_key = alice_keypair.secret;
        let alice_secret_key_x25519 = key_utils::ed25519_private_key_to_x25519(&alice_secret_key);
        let alice_public_key = alice_keypair.public;
        let alice_public_key_x25519 = PublicKey::from(&alice_secret_key_x25519);

        let encryptor = encryptor::Encryptor::new(vec![alice_public_key_x25519]);
        let message = encryptor.encrypt(plaintext, Some(&alice_secret_key));

        // create decryptor
        let decryptor = decryptor::Decryptor::new(&message);
        // get file key
        let file_key = decryptor
            .decrypt_file_key(&alice_secret_key_x25519)
            .expect("could decrypt file key");
        // check mac
        assert_eq!(decryptor.verify_message_mac(&file_key), true);
        // decrypt message payload
        let decrypted_plaintext = decryptor
            .decrypt_payload(&file_key)
            .expect("could decrypt payload");
        assert_eq!(decrypted_plaintext, plaintext);
        // verify signature
        let (mut sig_a, sig_b) = (
            message.meta.signature_a.to_vec(),
            message.meta.signature_b.to_vec(),
        );
        sig_a.extend_from_slice(&sig_b);
        assert!(decryptor::Decryptor::verify(
            &alice_public_key,
            &message.payload.ciphertext,
            &sig_a
        ));
    }
}
