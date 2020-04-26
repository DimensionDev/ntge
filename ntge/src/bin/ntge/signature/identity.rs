use clap::value_t;

use ntge_core::ed25519::{private::Ed25519PrivateKey, public::Ed25519PublicKey};
use std::fs;
use std::path::{Path, PathBuf};

use crate::{
    util::DEFAULT_SAVE_PATH,
};

#[derive(Debug, Clone)]
pub(crate) struct Identity {
    pub(crate) path: PathBuf,
    pub(crate) name: String,
    pub(crate) private_key: Option<Ed25519PrivateKey>,
    pub(crate) public_key: Option<Ed25519PublicKey>,
}

pub(crate) fn fetch_identity(arg_matches: &clap::ArgMatches) -> Option<Identity> {
    let request_identity_name =
        value_t!(arg_matches, "identity", String).unwrap_or_else(|e| e.exit());
    let local_identities = load_local_identities();

    if let Some(position) = local_identities
        .iter()
        .position(|i| i.name == request_identity_name)
    {
        Some(local_identities[position].clone())
    } else {
        // try to fetch name as path
        let path = Path::new(&request_identity_name);

        load_identity_at_path(&path.into())
    }
}

pub(crate) fn load_local_identities() -> Vec<Identity> {
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

    let mut identities: Vec<Identity> = vec![];
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };
        let path = entry.path();

        let identity = match load_identity_at_path(&path) {
            Some(identity) => identity,
            None => continue,
        };

        match identities.iter_mut().find(|i| i.name == identity.name) {
            Some(i) => {
                if i.private_key.is_none() {
                    i.private_key = identity.private_key.clone();
                } else if i.public_key.is_none() {
                    i.public_key = identity.public_key.clone();
                }
            }
            None => {
                identities.push(identity);
            }
        }
    } // end for entry in entries { … }

    identities
}

fn load_identity_at_path(path: &PathBuf) -> Option<Identity> {
    // init identity property
    let mut private_key: Option<Ed25519PrivateKey> = None;
    let mut public_key: Option<Ed25519PublicKey> = None;

    // get filename
    let name = match path.file_stem().and_then(|stem| stem.to_str()) {
        Some(name) => name.to_string(),
        None => return None,
    };
    // get file content
    let key_content = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(_) => return None,
    };

    // check file extension and set key
    if let Some(file_extension) = path.extension().and_then(|e| e.to_str()) {
        // have extension, skip if not "pub"
        if !file_extension.eq_ignore_ascii_case("pub") {
            return None;
        }

        let key = match Ed25519PublicKey::deserialize(key_content.trim()) {
            Ok(key) => key,
            Err(_) => return None,
        };
        public_key = Some(key);
    } else {
        // no extension, should be private key
        let key = match Ed25519PrivateKey::deserialize(key_content.trim()) {
            Ok(key) => key,
            Err(_) => return None,
        };
        private_key = Some(key);
    }

    if public_key.is_none() && private_key.is_none() {
        return None;
    }
    let identity = Identity {
        path: path.clone(),
        name: name.clone(),
        private_key,
        public_key,
    };

    Some(identity)
}

#[test]
fn it_load_local_identities() {
    let identities = load_local_identities();
    println!("{:?}", identities);
}
