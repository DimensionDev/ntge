use crate::signature::error::SignatureError;
use crate::util::{load_local_identities, Identity};
use clap::value_t;
use ntge_core::message::{decryptor::Decryptor, Message};

pub(crate) mod error;
pub(crate) mod identity;

pub(crate) fn fetch_signer(arg_matches: &clap::ArgMatches) -> Option<Identity> {
    match value_t!(arg_matches, "identity", String) {
        Ok(_) => match identity::fetch_identity(&arg_matches) {
            Some(identity) => Some(identity),
            None => {
                eprintln!("error: can not find identity to sign message");
                std::process::exit(1);
            }
        },
        Err(_) => None,
    }
}

pub(crate) fn fetch_verifier(arg_matches: &clap::ArgMatches) -> Vec<Identity> {
    match value_t!(arg_matches, "identity", String) {
        Ok(_) => match identity::fetch_identity(&arg_matches) {
            Some(identity) => vec![identity.clone()],
            None => {
                eprintln!("error: can not find identity to verify message");
                std::process::exit(1);
            }
        },
        Err(_) => load_local_identities(),
    }
}

pub(crate) fn verify_message_signature(
    plaintext: &str,
    identities: &[Identity],
) -> Result<Identity, SignatureError> {
    let message = if let Ok(it) = Message::deserialize_from_armor(plaintext.trim()) {
        it
    } else {
        return Err(SignatureError {
            message: format!("Can not read message"),
        });
    };

    if message.meta.signature.is_none() {
        return Err(SignatureError {
            message: format!("Can not find signature in the message"),
        });
    } else {
        for identity in identities.into_iter() {
            match &identity.public_key {
                Some(key) => {
                    if Decryptor::verify_signature(&message, &key) {
                        return Ok(identity.clone());
                    } else {
                        continue;
                    }
                }
                None => continue,
            }
        }
        Err(SignatureError {
            message: "Can not find valid public key to verify message signature.".to_string(),
        })
    }
}
