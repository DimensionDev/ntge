use chrono::prelude::*;
use hkdf::Hkdf;
use hmac::{Hmac, Mac};
use rand::rngs::OsRng;
use rand::RngCore;
use secrecy::ExposeSecret;
use sha2::Sha256;
use x25519_dalek::PublicKey;

use crate::{aead, message, x25519::FileKey};

#[derive(Debug)]
pub struct Encryptor {
    pub recipent_public_keys: Vec<PublicKey>,
}

impl Encryptor {
    pub fn new(recipent_public_keys: Vec<PublicKey>) -> Self {
        Encryptor {
            recipent_public_keys,
        }
    }
}

#[allow(dead_code)]
impl Encryptor {
    pub fn encrypt(&self, plaintext: &[u8]) -> message::Message {
        // 1. create new file key
        let file_key = FileKey::new();
        // 2. wrap file key for recipients
        let recipient_headers: Vec<_> = self
            .recipent_public_keys
            .iter()
            .map(|public_key| file_key.wrap(&public_key))
            .collect();
        let meta = message::MessageMeta {
            timestamp: Some(Utc::now().to_string()),
        };
        // 3. TODO: calculate HMAC for recipient_headers + meta
        let mac = Encryptor::calculate_mac(&recipient_headers, &meta, &file_key);
        // 4. create payload
        // prepare nonce
        let mut nonce = [0; 16];
        OsRng.fill_bytes(&mut nonce);
        // create payload key
        let mut payload_key = [0; 32];
        Hkdf::<Sha256>::new(Some(&nonce), file_key.0.expose_secret())
            .expand(message::PAYLOAD_KEY_LABEL, &mut payload_key)
            .expect("payload_key is the correct length");
        let ciphertext = aead::aead_encrypt(&payload_key, &plaintext);
        let payload = message::MessagePayload { nonce, ciphertext };
        // 5. construct message
        message::Message {
            recipient_headers,
            meta,
            mac,
            payload,
        }
    }
}

impl Encryptor {
    pub(crate) fn calculate_mac(
        recipient_headers: &[message::MessageRecipientHeader],
        meta: &message::MessageMeta,
        file_key: &FileKey,
    ) -> [u8; 32] {
        // create hmac key from file key
        let nonce = [0; 32];
        let mut mac_key = [0; 32];
        Hkdf::<Sha256>::new(Some(&nonce), file_key.0.expose_secret())
            .expand(message::MAC_KEY_LABEL, &mut mac_key)
            .expect("mac_key is the correct length");
        let mut hasher =
            Hmac::<Sha256>::new_varkey(&mac_key).expect("HMAC can take key of any size");
        for header in recipient_headers.iter() {
            hasher.input(header.key_type.as_bytes());
            hasher.input(header.ephemeral_public_key.as_bytes());
            hasher.input(&header.encrypted_file_key);
        }
        if let Some(timestamp) = &meta.timestamp {
            hasher.input(timestamp.as_bytes());
        }
        hasher.result().code().into()
    }
}
