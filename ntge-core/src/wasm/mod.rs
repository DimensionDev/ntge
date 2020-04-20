use wasm_bindgen::prelude::*;
use crate::ed25519;
use ed25519_dalek::{Keypair, PublicKey, SecretKey};

#[wasm_bindgen]
pub struct NTGEKeypair {
    _keypair: ed25519_dalek::Keypair
}

#[wasm_bindgen]
impl NTGEKeypair {
    pub fn new() -> NTGEKeypair {
        let inter_keypair = ed25519::create_keypair();
        NTGEKeypair {
            _keypair: inter_keypair
        }
    }

    pub fn get_public_key(&self) -> NTGEPublicKey {
        NTGEPublicKey {
            _pubkey: self._keypair.public
        }
    }

    pub fn get_secret_key(&self) -> Option<NTGESecretKey> {
        match ed25519_dalek::SecretKey::from_bytes(&(self._keypair.secret.to_bytes())).ok() {
            Some(sk) => Some(NTGESecretKey {
                _privkey: sk
            }),
            None => None
        }
    }
}

#[wasm_bindgen]
pub struct NTGEPublicKey {
    _pubkey: PublicKey
}

#[wasm_bindgen]
impl NTGEPublicKey {
    pub fn serialize(&self) -> String {
        // JsValue::from_serde(&self._pubkey).unwrap()
        ed25519::serialize_public_key(&self._pubkey)
    }
}

#[wasm_bindgen]
pub struct NTGESecretKey {
    _privkey: ed25519_dalek::SecretKey
}
