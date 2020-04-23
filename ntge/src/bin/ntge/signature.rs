use clap::value_t;
use ntge_core::{
    ed25519::public::Ed25519PublicKey,
    key_utils,
    message::{encryptor::Encryptor, Message},
    x25519::public::X25519PublicKey,
};

pub(crate) mod identity;

use crate::signature::identity::Identity;

pub(crate) fn fetch_signer(arg_matches: &clap::ArgMatches) -> Option<Identity> {
    match value_t!(arg_matches, "identity", String) {
        Ok(_) => {
            match identity::fetch_identity(&arg_matches) {
                Some(identity) => return Some(identity),
                None => {
                    eprintln!("error: can not find identity to sign message");
                    std::process::exit(1);
                }
            };
        }
        Err(_) => return None,
    };
}
