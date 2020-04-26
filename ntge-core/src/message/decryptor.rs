use ed25519_dalek::Signature;
use hkdf::Hkdf;
use secrecy::ExposeSecret;
use sha2::Sha256;

use crate::{
    aead,
    buffer::Buffer,
    ed25519,
    ed25519::public::Ed25519PublicKey,
    message::{self, encryptor::Encryptor, Message},
    x25519::filekey::FileKey,
    x25519::private::X25519PrivateKey,
};

#[derive(Debug)]
pub struct Decryptor {
    pub message: message::Message,
}

impl Decryptor {
    pub fn new(message: &message::Message) -> Self {
        Decryptor {
            message: message.clone(),
        }
    }
}

impl Decryptor {
    pub fn verify_message_mac(&self, file_key: &FileKey) -> bool {
        let calculated_mac = Encryptor::calculate_mac(
            &self.message.recipient_headers,
            &self.message.meta,
            &file_key,
        );
        let calculated_mac = calculated_mac.to_vec();
        calculated_mac == self.message.mac.mac
    }

    pub fn decrypt_file_key(&self, private_key: &X25519PrivateKey) -> Option<FileKey> {
        for header in self.message.recipient_headers.iter() {
            match FileKey::unwrap(&header, &private_key) {
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

impl Decryptor {
    pub fn verify_signature(message: &message::Message, public_key: &Ed25519PublicKey) -> bool {
        let signature_bytes = match &message.meta.signature {
            Some(signature) => &signature.data,
            None => return false,
        };

        let signature: Signature = match Signature::from_bytes(&signature_bytes[..]) {
            Ok(sig) => sig,
            Err(_) => return false,
        };

        match ed25519::public::verify(&public_key.raw, &message.payload.ciphertext, &signature) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

impl Drop for Decryptor {
    fn drop(&mut self) {
        if cfg!(feature = "drop-log-enable") {
            println!("{:?} is being deallocated", self);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn c_message_decryptor_destroy(decryptor: *mut Decryptor) {
    let _ = Box::from_raw(decryptor);
}

#[no_mangle]
pub unsafe extern "C" fn c_message_decryptor_new(message: *mut message::Message) -> *mut Decryptor {
    let message = &mut *message;
    let decryptor = Decryptor::new(&message);
    Box::into_raw(Box::new(decryptor))
}

#[no_mangle]
pub unsafe extern "C" fn c_message_decryptor_verify_message_mac(
    decryptor: *mut Decryptor,
    file_key: *mut FileKey,
) -> bool {
    let decryptor = &mut *decryptor;
    let file_key = &mut *file_key;
    decryptor.verify_message_mac(&file_key)
}

#[no_mangle]
pub unsafe extern "C" fn c_message_decryptor_decrypt_file_key(
    decryptor: *mut Decryptor,
    private_key: *mut X25519PrivateKey,
) -> *mut FileKey {
    let decryptor = &mut *decryptor;
    let private_key = &mut *private_key;
    match decryptor.decrypt_file_key(private_key) {
        Some(file_key) => Box::into_raw(Box::new(file_key)),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn c_message_decryptor_decrypt_payload(
    decryptor: *mut Decryptor,
    file_key: *mut FileKey,
) -> Buffer {
    let decryptor = &mut *decryptor;
    let file_key = &mut *file_key;
    match decryptor.decrypt_payload(&file_key) {
        Some(bytes) => {
            let slice = bytes.into_boxed_slice();
            let data = slice.as_ptr();
            let len = slice.len();
            std::mem::forget(slice);
            Buffer {
                data: data,
                len: len,
            }
        }
        None => Buffer {
            data: std::ptr::null_mut(),
            len: 0,
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn c_message_decryptor_verify_signature(
    message: *mut Message,
    public_key: *mut Ed25519PublicKey,
) -> bool {
    let message = &mut *message;
    let public_key = &mut *public_key;

    Decryptor::verify_signature(&message, &public_key)
}
