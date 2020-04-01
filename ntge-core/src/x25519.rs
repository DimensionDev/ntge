// We re-write message encryption logic here from rage by @str4d
// https://github.com/str4d/rage

use hkdf::Hkdf;
use rand::rngs::OsRng;
use rand::RngCore;
use secrecy::{ExposeSecret, Secret};
use sha2::Sha256;
use x25519_dalek::{EphemeralSecret, PublicKey, StaticSecret};

use crate::{aead, error::CoreError, message::MessageRecipientHeader};

pub const CURVE_NAME_X25519: &str = "X25519";
pub(crate) const ENCRYPTED_FILE_KEY_BYTES: usize = 32;
const X25519_RECIPIENT_KEY_LABEL: &[u8] = b"ntge-encryption-X25519/v1";

#[derive(Debug)]
pub struct FileKey(pub(crate) Secret<[u8; 16]>);

#[allow(dead_code)]
impl FileKey {
    pub(crate) fn new() -> Self {
        let mut file_key = [0; 16];
        OsRng.fill_bytes(&mut file_key);
        FileKey(Secret::new(file_key))
    }
}

#[allow(dead_code)]
impl FileKey {
    // wrap file key by public key
    pub(crate) fn wrap(&self, public_key: &PublicKey) -> MessageRecipientHeader {
        // 1. create ephemeral x25519 key
        let mut csprng = OsRng {};
        let ephemeral_private_key = EphemeralSecret::new(&mut csprng);
        let ephemeral_public_key = PublicKey::from(&ephemeral_private_key);
        // 2. shared_secret = ECHD(ephemeral_private_key, public_key)
        let shared_secret = ephemeral_private_key.diffie_hellman(&public_key);
        // 3. use ephemeral_public_key appending public_key as salt
        let mut salt = vec![];
        salt.extend_from_slice(ephemeral_public_key.as_bytes());
        salt.extend_from_slice(public_key.as_bytes());
        // 4. encryption_key = HKDF(shared_secret)
        let mut encryption_key = [0; 32];
        Hkdf::<Sha256>::new(Some(&salt), shared_secret.as_bytes())
            .expand(X25519_RECIPIENT_KEY_LABEL, &mut encryption_key)
            .expect("encryption_key is the correct length");
        // 5. encrypt file key
        let encrypted_file_key = {
            let mut key = [0; ENCRYPTED_FILE_KEY_BYTES];
            key.copy_from_slice(&aead::aead_encrypt(&encryption_key, self.0.expose_secret()));
            key
        };
        // 6. create MessageRecipient
        MessageRecipientHeader {
            key_type: CURVE_NAME_X25519,
            ephemeral_public_key,
            encrypted_file_key,
        }
    }

    pub(crate) fn unwrap(
        message_recipient: &MessageRecipientHeader,
        secret_key: &StaticSecret,
    ) -> Result<Self, CoreError> {
        // 1. calculate ECDH shared_secret
        let shared_secret = secret_key.diffie_hellman(&message_recipient.ephemeral_public_key);
        // 2. create public key from private key
        let public_key = PublicKey::from(secret_key);
        // 3. calculate same salt in wrap method
        let mut salt = vec![];
        salt.extend_from_slice(message_recipient.ephemeral_public_key.as_bytes());
        salt.extend_from_slice(public_key.as_bytes());
        // 4. calculate same encryption key in wrap method
        let mut encryption_key = [0; 32];
        Hkdf::<Sha256>::new(Some(&salt), shared_secret.as_bytes())
            .expand(X25519_RECIPIENT_KEY_LABEL, &mut encryption_key)
            .expect("encryption_key is the correct length");
        // 5. decrypt file key
        match aead::aead_decrypt(&encryption_key, &message_recipient.encrypted_file_key) {
            Ok(key) => {
                let mut file_key = [0; 16];
                file_key.copy_from_slice(&key);
                Ok(FileKey(Secret::new(file_key)))
            }
            Err(_) => {
                let e = CoreError::MessageDecryptionError {
                    name: "Message",
                    reason: "cannot decrypt file key from message recipient",
                };
                Err(e)
            }
        }
    }
}

#[test]
fn it_smoke() {
    let mut alice_csprng = OsRng {};
    let alice_private = EphemeralSecret::new(&mut alice_csprng);
    let _alice_public = PublicKey::from(&alice_private);

    let mut bob_csprng = OsRng {};
    let bob_secret = EphemeralSecret::new(&mut bob_csprng);
    let _bob_public = PublicKey::from(&bob_secret);
}

#[test]
fn it_use_aead_encrypt_and_decrypt() {
    // create ChaCha20 key
    let mut csprng = OsRng {};
    let mut key = [0; 32];
    csprng.fill_bytes(&mut key);

    let plaintext = "Plaintext";
    // aead encrypt plaintext
    let ciphertext = aead::aead_encrypt(&key, &plaintext.as_bytes());
    // aead decrypt ciphertext
    let decrypted_vec = aead::aead_decrypt(&key, &ciphertext).unwrap();
    // expect decrypted text same to original plaintext
    let plaintext_vec = plaintext.as_bytes().to_vec();
    assert_eq!(plaintext_vec, decrypted_vec);
}

#[test]
fn it_wrap_then_unwrap_a_file_key() {
    // file_key for recipient
    let file_key = FileKey::new();
    // alice
    let mut alice_csprng = OsRng {};
    let alice_secret_key = StaticSecret::new(&mut alice_csprng);
    let alice_public_key = PublicKey::from(&alice_secret_key);
    // wrap file key
    let message_recipient_for_alice = file_key.wrap(&alice_public_key);
    // unwrap it
    let decrypted_file_key =
        FileKey::unwrap(&message_recipient_for_alice, &alice_secret_key).unwrap();
    // expect same file key after unwrap
    assert_eq!(
        file_key.0.expose_secret(),
        decrypted_file_key.0.expose_secret()
    );
}
