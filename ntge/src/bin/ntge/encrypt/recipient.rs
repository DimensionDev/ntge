use clap::values_t;
use ntge_core::ed25519::{public::Ed25519PublicKey};
use std::fs;
use std::path::{Path, PathBuf};

use crate::util::DEFAULT_SAVE_PATH;

#[derive(Debug, Clone)]
pub(crate) struct Recipient {
    pub(crate) path: PathBuf,
    pub(crate) name: String,
    pub(crate) key_content: String,
    pub(crate) key: Ed25519PublicKey,
}

pub(crate) fn fetch_recipient(arg_matches: &clap::ArgMatches) -> Vec<Recipient> {
    let request_recipient_names =
        clap::values_t!(arg_matches, "recipient", String).unwrap_or_else(|e| e.exit());
    let local_recipients = load_local_recipients();
    let mut recipients = vec![];

    for name in request_recipient_names {
        if let Some(position) = local_recipients.iter().position(|r| r.name == name) {
            // local recipients contains that name
            recipients.push(local_recipients[position].clone());
        } else {
            // try to fetch name as path
            let path = Path::new(&name);
            let key_content = match fs::read_to_string(&path) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("error: can not read recipient {}.\nreason: {}", name, e);
                    std::process::exit(1);
                }
            };
            let key = match Ed25519PublicKey::deserialize(key_content.trim()) {
                Ok(key) => key,
                Err(e) => {
                    eprintln!("error: can not parse recipient {}.\nreason: {}", name, e);
                    std::process::exit(1);
                }
            };
            let recipient = Recipient {
                path: path.into(),
                name,
                key_content,
                key,
            };
            recipients.push(recipient);
        }
    }

    recipients
}

pub(crate) fn load_local_recipients() -> Vec<Recipient> {
    // find HOME
    let home = match dirs::home_dir() {
        Some(dir) => dir,
        None => return vec![],
    };
    // find .ntge
    let folder_path = home.join(Path::new(DEFAULT_SAVE_PATH));
    if !folder_path.is_dir() || !folder_path.exists() {
        return vec![];
    }
    let entries = match fs::read_dir(folder_path) {
        Ok(entries) => entries,
        Err(_) => return vec![],
    };

    let mut recipients = vec![];
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };
        let path = entry.path();
        // get file extension
        let file_extension = match path.extension() {
            Some(extension) => match extension.to_str() {
                Some(extension) => extension,
                None => continue,
            },
            None => continue,
        };
        // skip if not "pub"
        if !file_extension.eq_ignore_ascii_case("pub") {
            continue;
        }
        // get filename
        let name = match path.file_stem() {
            Some(stem) => match stem.to_str() {
                Some(name) => name.to_string(),
                None => continue,
            },
            None => continue,
        };
        // get public key from file
        let key_content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(_) => continue,
        };
        let key = match Ed25519PublicKey::deserialize(key_content.trim()) {
            Ok(key) => key,
            Err(_) => continue,
        };

        let recipient = Recipient {
            path,
            name,
            key_content,
            key,
        };
        recipients.push(recipient);
    } // end for entry in entries { … }

    recipients
}
