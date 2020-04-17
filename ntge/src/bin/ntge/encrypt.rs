use ntge_core::ed25519::public::Ed25519PublicKey;
use ntge_core::key_utils;
use ntge_core::message::{encryptor::Encryptor, Message};
use ntge_core::x25519::public::X25519PublicKey;
use recipient::Recipient;

pub mod recipient;

pub(crate) fn encrypt_message(plaintext: &[u8], recipients: &[Recipient]) -> Message {
    let keys: Vec<_> = recipients
        .iter()
        .map(|r| key_utils::ed25519_public_key_to_x25519(&Ed25519PublicKey { raw: r.key }))
        .collect();

    let encryptor = Encryptor::new(&keys[..]);
    let message = encryptor.encrypt(plaintext);

    message
}
