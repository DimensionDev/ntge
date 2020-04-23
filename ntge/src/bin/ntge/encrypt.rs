use ntge_core::ed25519::public::Ed25519PublicKey;
use ntge_core::key_utils;
use ntge_core::message::{encryptor::Encryptor, Message};
use ntge_core::x25519::public::X25519PublicKey;

use crate::{encrypt::recipient::Recipient, signature::identity::Identity};

pub mod recipient;

pub(crate) fn encrypt_message(
    plaintext: &[u8],
    recipients: &[Recipient],
    singer: Option<&Identity>,
) -> Message {
    let keys: Vec<_> = recipients
        .iter()
        .map(|r| X25519PublicKey::from(&r.key))
        .collect();
    let signature_key = singer.and_then(|s| s.private_key.clone());

    let encryptor = Encryptor::new(&keys[..]);
    let message = encryptor.encrypt(plaintext, signature_key.as_ref());

    message
}
