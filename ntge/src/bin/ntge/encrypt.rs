use ntge_core::key_utils;
use ntge_core::message::{encryptor::Encryptor, Message};
use recipient::Recipient;

pub mod recipient;

pub(crate) fn encrypt_message(plaintext: &[u8], recipients: &[Recipient]) -> Message {
    let keys = recipients
        .iter()
        .map(|r| key_utils::ed25519_public_key_to_x25519(&r.key))
        .collect();

    let encryptor = Encryptor::new(keys);
    let message = encryptor.encrypt(plaintext);

    message
}
