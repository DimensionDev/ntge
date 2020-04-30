use crate::encrypt::recipient::Recipient;
use crate::util::Identity;
use ntge_core::message::{encryptor::Encryptor, Message};
use ntge_core::x25519::public::X25519PublicKey;

pub mod recipient;

pub(crate) fn encrypt_message(
    plaintext: &[u8],
    recipients: &[Recipient],
    signer: Option<&Identity>,
) -> Message {
    let keys: Vec<_> = recipients
        .iter()
        .map(|r| X25519PublicKey::from(&r.key))
        .collect();
    let signature_key = signer.and_then(|s| s.private_key.clone());
    let encryptor = Encryptor::new(&keys[..]);

    encryptor.encrypt(plaintext, signature_key.as_ref())
}
