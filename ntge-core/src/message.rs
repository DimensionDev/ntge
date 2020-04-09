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
    fn serialize_to_base58(&self) -> Result<String, CoreError> {
        self.serialize_to_bson_bytes()
            .map(|bytes| bs58::encode(bytes).into_string())
            .map_err(|_| CoreError::MessageSerializationError {
                name: "Message",
                reason: "cannot encode message bytes to base58",
            })
    }

    fn deserialize_from_base58(text: &str) -> Result<Message, CoreError> {
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

    #[allow(clippy::ptr_arg)]
    fn deserialize_from_bson_bytes(bytes: &Vec<u8>) -> Result<Message, CoreError> {
        let document = bson::decode_document(&mut std::io::Cursor::new(&bytes[..]))?;

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
    fn it_encrypt_message_to_alice() {
        // plaintext
        let plaintext = b"Hello, World!";
        // create new file key
        let file_key = FileKey::new();
        // pass file key to encrypt plaintext
        let ciphertext = encrypt_plaintext(&file_key, plaintext);
        print!("{:?}", ciphertext);
    }
    #[test]
    fn it_encrypt_and_decrypt_message_to_alice() {
        let plaintext = b"Hello, World!";
        // alice
        let mut alice_csprng = OsRng {};
        let alice_secret_key = StaticSecret::new(&mut alice_csprng);
        let alice_public_key = PublicKey::from(&alice_secret_key);

        let encryptor = encryptor::Encryptor::new(vec![alice_public_key]);
        let message = encryptor.encrypt(plaintext);

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
    fn it_encode_and_decode_message_to_alice() {
        let plaintext = b"Hello, World!";

        // alice
        let mut alice_csprng = OsRng {};
        let alice_secret_key = StaticSecret::new(&mut alice_csprng);
        let alice_public_key = PublicKey::from(&alice_secret_key);

        let encryptor = encryptor::Encryptor::new(vec![alice_public_key]);
        let message = encryptor.encrypt(plaintext);
        // encode
        let encoded_message = message.serialize_to_base58().expect("could serialize");
        print!("{:?}", encoded_message);
        let decoded_message =
            Message::deserialize_from_base58(&encoded_message).expect("could deserialize");
        assert_eq!(decoded_message.mac.mac, message.mac.mac);
        assert_eq!(decoded_message.meta.timestamp, message.meta.timestamp);
    }
}
