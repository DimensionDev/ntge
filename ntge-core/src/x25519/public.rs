use crate::{ed25519::public::Ed25519PublicKey, key_utils};

#[derive(Debug)]
pub struct X25519PublicKey {
    pub raw: x25519_dalek::PublicKey,
}

impl Clone for X25519PublicKey {
    fn clone(&self) -> Self {
        X25519PublicKey {
            raw: self.raw.clone(),
        }
    }
}

impl Drop for X25519PublicKey {
    fn drop(&mut self) {
        if cfg!(feature = "drop-log-enable") {
            println!("{:?} is being deallocated", self);
        }
    }
}

impl From<&Ed25519PublicKey> for X25519PublicKey {
    fn from(public_key: &Ed25519PublicKey) -> X25519PublicKey {
        X25519PublicKey {
            raw: key_utils::ed25519_public_key_to_x25519(&public_key.raw),
        }
    }
}

#[no_mangle]
#[cfg(target_os = "ios")]
pub extern "C" fn c_x25519_public_key_destroy(public_key: &mut X25519PublicKey) {
    let _ = unsafe { Box::from_raw(public_key) };
}
