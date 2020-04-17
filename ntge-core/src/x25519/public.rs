use x25519_dalek;

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
        println!("{:?} is being deallocated", self);
    }
}

#[no_mangle]
pub extern "C" fn c_x25519_public_key_destroy(public_key: &mut X25519PublicKey) {
    let _ = unsafe { Box::from_raw(public_key) };
}
