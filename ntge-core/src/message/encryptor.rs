use chrono::prelude::*;
use hkdf::Hkdf;
use hmac::{Hmac, Mac};
use rand::rngs::OsRng;
use rand::RngCore;
use secrecy::ExposeSecret;
use sha2::Sha256;
use x25519_dalek;

use crate::{
    aead, arrays,
    buffer::Buffer,
    message,
    x25519::{filekey::FileKey, public::X25519PublicKey},
};

#[derive(Debug)]
pub struct Encryptor {
    pub x25519_public_keys: Vec<X25519PublicKey>,
}

impl Encryptor {
    pub fn new(x25519_public_keys: &[X25519PublicKey]) -> Self {
        Encryptor {
            x25519_public_keys: x25519_public_keys.to_vec(),
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
            .x25519_public_keys
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
        let nonce = nonce.to_vec();
        let payload = message::MessagePayload { nonce, ciphertext };
        // 5. construct message
        let mac = message::MessageMac { mac: mac.to_vec() };
        message::Message {
            recipient_headers,
            meta,
            mac,
            payload,
        }
    }
}

impl Encryptor {
    pub(super) fn calculate_mac(
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
            hasher.input(header.get_ephemeral_public_key().as_bytes());
            hasher.input(&header.encrypted_file_key);
        }
        if let Some(timestamp) = &meta.timestamp {
            hasher.input(timestamp.as_bytes());
        }
        hasher.result().code().into()
    }
}

impl Drop for Encryptor {
    fn drop(&mut self) {
        println!("{:?} is being deallocated", self);
    }
}

#[no_mangle]
pub extern "C" fn c_array_new_for_x25519_public_key() -> *mut Vec<X25519PublicKey> {
    let array: Vec<X25519PublicKey> = arrays::new_array();
    Box::into_raw(Box::new(array))
}

#[no_mangle]
pub unsafe extern "C" fn c_array_destroy_x25519_public_key(public_keys: *mut Vec<X25519PublicKey>) {
    let _ = Box::from_raw(public_keys);
}

#[no_mangle]
pub unsafe extern "C" fn c_array_push_x25519_public_key(
    array: *mut Vec<X25519PublicKey>,
    element: *mut X25519PublicKey,
) {
    let array = &mut *array;
    let element = &mut *element;
    arrays::push_to(array, element);
}

#[no_mangle]
pub unsafe extern "C" fn c_message_encryptor_new(
    x25519_public_keys: *mut Vec<X25519PublicKey>,
) -> *mut Encryptor {
    let x25519_public_keys = &mut *x25519_public_keys;
    let encryptor = Encryptor::new(&x25519_public_keys);
    Box::into_raw(Box::new(encryptor))
}

#[no_mangle]
pub unsafe extern "C" fn c_message_encryptor_destroy(encryptor: *mut Encryptor) {
    let _ = Box::from_raw(encryptor);
}

#[no_mangle]
pub unsafe extern "C" fn c_message_encryptor_encrypt_plaintext(
    encryptor: *mut Encryptor,
    plaintext_buffer: Buffer,
) -> *mut message::Message {
    let encryptor = &mut *encryptor;
    let data = plaintext_buffer.to_bytes();
    let message = encryptor.encrypt(&data[..]);
    Box::into_raw(Box::new(message))
}
