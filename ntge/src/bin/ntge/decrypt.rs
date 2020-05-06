use crate::decrypt::error::DecryptError;
use crate::util::{load_local_identities, Identity};
use clap::value_t;
use ntge_core::key_utils::ed25519_private_key_to_x25519;
use ntge_core::message::{decryptor::Decryptor, Message};
use ntge_core::x25519::private::X25519PrivateKey;

mod error;
mod identity;

pub(crate) fn decrypt_message(
    ciphertext: &str,
    identities: &[Identity],
) -> Result<Vec<u8>, DecryptError> {
    let message = if let Ok(it) = Message::deserialize_from_armor(ciphertext.trim()) {
        it
    } else {
        return Err(DecryptError {
            message: "Can not read message".to_string(),
        });
    };
    let decryptor = Decryptor::new(&message);
    for identity in identities.iter() {
        match &identity.private_key {
            Some(key) => {
                let private_key = X25519PrivateKey {
                    raw: ed25519_private_key_to_x25519(&key.raw),
                };
                let filekey = match decryptor.decrypt_file_key(&private_key) {
                    Some(fk) => fk,
                    None => continue,
                };
                if !decryptor.verify_message_mac(&filekey) {
                    continue;
                }
                let plaintext = match decryptor.decrypt_payload(&filekey) {
                    Some(pt) => pt,
                    None => {
                        return Err(DecryptError {
                            message: "Decryption Failed.".to_string(),
                        })
                    }
                };
                return Ok(plaintext);
            }
            None => break,
        };
    }

    Err(DecryptError {
        message: "Decryption Failed,".to_string(),
    })
}

pub(crate) fn fetch_decryptor(arg_matches: &clap::ArgMatches) -> Vec<Identity> {
    match value_t!(arg_matches, "identity", String) {
        Ok(_) => match identity::fetch_identity(&arg_matches) {
            Some(identity) => vec![identity],
            None => {
                eprintln!("error: can not find identity to verify message");
                std::process::exit(1);
            }
        },
        Err(_) => load_local_identities(),
    }
}
