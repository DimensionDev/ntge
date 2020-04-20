use wasm_bindgen::prelude::*;
extern crate js_sys;
extern crate web_sys;
use web_sys::console;
use crate::ed25519;
use ed25519_dalek::{Keypair, PublicKey, SecretKey};

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct NTGEKeypair {
    _keypair: ed25519_dalek::Keypair
}

#[wasm_bindgen]
impl NTGEKeypair {
    pub fn new() -> NTGEKeypair {
        log!("rust::Creating");
        console_error_panic_hook::set_once();
        let inter_keypair = ed25519::create_keypair();
        log!("rust::Created");
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
