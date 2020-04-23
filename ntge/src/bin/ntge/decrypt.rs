use crate::decrypt::error::DecryptError;
use ntge_core::ed25519::deserialize_private_key;
use ntge_core::ed25519::SecretKey;
use ntge_core::key_utils::ed25519_private_key_to_x25519;
use ntge_core::message::{decryptor::Decryptor, Message};
use ntge_core::x25519::private::X25519PrivateKey;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;

mod error;

const DEFAULT_SAVE_PATH: &str = ".ntge";

#[allow(dead_code)]
pub(crate) struct DecryptResult {
    pub(crate) content: Vec<u8>,
    pub(crate) key_content: String,
    pub(crate) key: X25519PrivateKey,
}

fn try_decrypt_with_key(
    decryptor: &Decryptor,
    identity_path: &Path,
) -> Result<DecryptResult, DecryptError> {
    if !identity_path.exists() {
        return Err(DecryptError {
            message: format!(
                "Can not find identity file from {}",
                identity_path.display()
            ),
        });
    }
    if !identity_path.is_file() {
        return Err(DecryptError {
            message: format!("{} is not a file", identity_path.display()),
        });
    }
    let mut file = if let Ok(it) = File::open(&identity_path) {
        it
    } else {
        return Err(DecryptError {
            message: format!("Can not open file {}", identity_path.display()),
        });
    };
    let mut key_contents = String::new();
    if let Err(_) = file.read_to_string(&mut key_contents) {
        return Err(DecryptError {
            message: format!("Can not read file {}", identity_path.display()),
        });
    }
    let private_key = if let Ok(it) = deserialize_private_key(key_contents.as_ref()) {
        it
    } else {
        return Err(DecryptError {
            message: format!("{} is not a vaild private key", identity_path.display()),
        });
    };

    let secret_key = ed25519_private_key_to_x25519(&private_key);
    let private_key = X25519PrivateKey { raw: secret_key };
    let file_key = if let Some(it) = decryptor.decrypt_file_key(&private_key) {
        it
    } else {
        return Err(DecryptError {
            message: format!("Can not get decrupt file key"),
        });
    };
    let result = if let Some(it) = decryptor.decrypt_payload(&file_key) {
        it
    } else {
        return Err(DecryptError {
            message: format!("Decrypt failure"),
        });
    };
    return Ok(DecryptResult {
        content: result,
        key: private_key,
        key_content: key_contents,
    });
}

fn load_local_keys() -> Vec<PathBuf> {
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

    let mut items = vec![];
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };
        let path = entry.path();
        if path.extension() != None {
            continue;
        }

        items.push(path);
    }
    items
}

pub(crate) fn decrypt_message(
    plaintext: &str,
    identity: Option<&str>,
) -> Result<DecryptResult, DecryptError> {
    let message = if let Ok(it) = Message::deserialize_from_armor(plaintext.trim()) {
        it
    } else {
        return Err(DecryptError {
            message: format!("Can not decript message"),
        });
    };
    let decryptor = Decryptor::new(&message);
    if let Some(id) = identity {
        let identity_path = Path::new(id);
        try_decrypt_with_key(&decryptor, identity_path)
    } else {
        let keys = load_local_keys();
        for item in keys {
            let result = try_decrypt_with_key(&decryptor, &item);
            if result.is_ok() {
                return result;
            }
        }
        return Err(DecryptError {
            message: format!("Can not find key for the message"),
        });
    }
}
