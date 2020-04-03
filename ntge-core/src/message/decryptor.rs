use hkdf::Hkdf;
use secrecy::ExposeSecret;
use sha2::Sha256;
use x25519_dalek::StaticSecret;

use crate::{
    aead,
    message::{self, encryptor::Encryptor},
    x25519::FileKey,
};

#[derive(Debug)]
pub struct Decryptor<'a> {
    pub message: &'a message::Message,
}

impl<'a> Decryptor<'a> {
    pub fn new(message: &'a message::Message) -> Self {
        Decryptor { message }
    }
}

impl<'a> Decryptor<'a> {
    pub fn verify_message_mac(&self, file_key: &FileKey) -> bool {
        let calculated_mac = Encryptor::calculate_mac(
            &self.message.recipient_headers,
            &self.message.meta,
            &file_key,
        );
        let calculated_mac = calculated_mac.to_vec();
        calculated_mac == self.message.mac.mac
    }

    pub fn decrypt_file_key(&self, secret_key: &StaticSecret) -> Option<FileKey> {
        for header in self.message.recipient_headers.iter() {
            match FileKey::unwrap(&header, &secret_key) {
                Ok(file_key) => {
                    return Some(file_key);
                }
                Err(_) => {
                    continue;
                }
            }
        }
        None
    }

    pub fn decrypt_payload(&self, file_key: &FileKey) -> Option<Vec<u8>> {
        let nonce = &self.message.payload.nonce;
        // create payload key
        let mut payload_key = [0; 32];
        Hkdf::<Sha256>::new(Some(&nonce), file_key.0.expose_secret())
            .expand(message::PAYLOAD_KEY_LABEL, &mut payload_key)
            .expect("payload_key is the correct length");
        match aead::aead_decrypt(&payload_key, &self.message.payload.ciphertext) {
            Ok(plaintext) => Some(plaintext),
            Err(_) => None,
        }
    }
}
