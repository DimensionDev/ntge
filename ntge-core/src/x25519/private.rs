use ed25519_dalek;
use std::fmt;

use crate::{ed25519::private::Ed25519PrivateKey, key_utils};

pub struct X25519PrivateKey {
    pub raw: x25519_dalek::StaticSecret,
}

impl Clone for X25519PrivateKey {
    fn clone(&self) -> Self {
        X25519PrivateKey {
            raw: self.raw.clone(),
        }
    }
}

impl fmt::Debug for X25519PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("X25519PrivateKey").finish()
    }
}

impl From<&Ed25519PrivateKey> for X25519PrivateKey {
    fn from(private_key: &Ed25519PrivateKey) -> X25519PrivateKey {
        X25519PrivateKey {
            raw: key_utils::ed25519_private_key_to_x25519(&private_key.raw),
        }
    }
}

impl Drop for X25519PrivateKey {
    fn drop(&mut self) {
        println!("The X25519PrivateKey is being deallocated");
    }
}

#[no_mangle]
pub extern "C" fn c_x25519_private_key_destroy(private_key: &mut X25519PrivateKey) {
    let _ = unsafe { Box::from_raw(private_key) };
}
