use ed25519_dalek;
use std::fmt;

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

impl Drop for X25519PrivateKey {
    fn drop(&mut self) {
        println!("The X25519PrivateKey is being deallocated");
    }
}

#[no_mangle]
pub extern "C" fn c_x25519_private_key_destroy(private_key: &mut X25519PrivateKey) {
    let _ = unsafe { Box::from_raw(private_key) };
}
