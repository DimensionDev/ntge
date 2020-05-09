use ntge_core::ed25519::{private::Ed25519PrivateKey, public::Ed25519PublicKey};
use std::fs;
use std::io::Write;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

pub(crate) const DEFAULT_SAVE_PATH: &str = ".ntge";

// read input from stdin or path
pub(crate) fn read_input_bytes(arg_matches: &clap::ArgMatches) -> Vec<u8> {
    if let Some(path) = arg_matches.value_of("path") {
        // read from file
        let file_path = Path::new(path);
        match fs::read(&file_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("error: can not read file {}.\nreason: {}", path, e);
                std::process::exit(1);
            }
        }
    } else {
        // read from stdin
        let mut input = String::new();
        match io::stdin().read_to_string(&mut input) {
            Ok(_) => input.as_bytes().to_vec(),
            Err(e) => {
                eprintln!("error: can not read content from stdin.\nreason: {}", e);
                std::process::exit(1);
            }
        }
    }
}

// read input from stdin or path
pub(crate) fn read_input_str(arg_matches: &clap::ArgMatches) -> String {
    if let Some(path) = arg_matches.value_of("path") {
        // read from file
        let file_path = Path::new(path);
        match fs::read_to_string(&file_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("error: can not read file {}.\nreason: {}", path, e);
                std::process::exit(1);
            }
        }
    } else {
        // read from stdin
        let mut input = String::new();
        match io::stdin().read_to_string(&mut input) {
            Ok(_) => input.as_str().to_string(),
            Err(e) => {
                eprintln!("error: can not read content from stdin.\nreason: {}", e);
                std::process::exit(1);
            }
        }
    }
}

pub(crate) fn write_to_output(arg_matches: &clap::ArgMatches, content: &[u8]) {
    if let Some(path) = arg_matches.value_of("output") {
        // write to file
        let file_path = Path::new(path);
        let mut file = match fs::File::create(&file_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("error: can not create file at path {}\nreason: {}", path, e);
                std::process::exit(1);
            }
        };
        match file.write_all(content) {
            Ok(_) => {
                // add trailing line break
                let _ = file.write_all(b"\n");
            }
            Err(e) => {
                eprintln!("error: can not write to file {}\nreason: {}", path, e);
                std::process::exit(1);
            }
        };
    } else {
        // write to stdout
        match io::stdout().lock().write_all(content) {
            Ok(_) => {
                // add trailing line break
                let _ = io::stdout().lock().write_all(b"\n");
            }
            Err(e) => {
                eprintln!("error: can not write to stdout\nreason: {}", e);
                std::process::exit(1);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Identity {
    pub(crate) path: PathBuf,
    pub(crate) name: String,
    pub(crate) private_key: Option<Ed25519PrivateKey>,
    pub(crate) public_key: Option<Ed25519PublicKey>,
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

pub fn load_identity_at_path(path: &PathBuf) -> Option<Identity> {
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

    Some(Identity {
        path: path.clone(),
        name,
        private_key,
        public_key,
    })
}
