use hkdf::Hkdf;
use rand::rngs::OsRng;
use rand::RngCore;
use secrecy::ExposeSecret;
use sha2::Sha256;

use crate::{
    aead,
    x25519::{self, FileKey},
};

pub mod decryptor;
pub mod encryptor;

pub(crate) const MAC_KEY_LABEL: &[u8] = b"ntge-message-mac-key";
pub(crate) const PAYLOAD_KEY_LABEL: &[u8] = b"ntge-message-payload";

#[derive(Debug)]
pub struct MessageRecipientHeader {
    pub key_type: &'static str,
    pub ephemeral_public_key: x25519_dalek::PublicKey,
    pub encrypted_file_key: [u8; x25519::ENCRYPTED_FILE_KEY_BYTES],
}

#[derive(Debug)]
pub struct MessageMeta {
    pub timestamp: Option<String>,
}

#[derive(Debug)]
pub struct MessagePayload {
    pub nonce: [u8; 16],
    pub ciphertext: Vec<u8>,
}

#[derive(Debug)]
pub struct Message {
    pub recipient_headers: Vec<MessageRecipientHeader>,
    pub meta: MessageMeta,
    pub mac: [u8; 32],
    pub payload: MessagePayload,
}

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

#[cfg(test)]
mod tests {
    use crate::message::{decryptor, encryptor};
    use rand::rngs::OsRng;
    use x25519_dalek::{PublicKey, StaticSecret};
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
}
