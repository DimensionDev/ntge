pub use ed25519_dalek::{self, PublicKey, SecretKey};
use rand::rngs::OsRng;

use crate::{ed25519::private::Ed25519PrivateKey, ed25519::public::Ed25519PublicKey};

#[derive(Debug)]
pub struct Ed25519Keypair {
    pub raw: ed25519_dalek::Keypair,
}

impl Ed25519Keypair {
    pub fn get_private_key(&self) -> Ed25519PrivateKey {
        Ed25519PrivateKey {
            raw: SecretKey::from_bytes(&self.raw.secret.to_bytes()).unwrap(),
        }
    }

    pub fn get_public_key(&self) -> Ed25519PublicKey {
        Ed25519PublicKey {
            raw: PublicKey::from_bytes(&self.raw.public.to_bytes()).unwrap(),
        }
    }
}

impl Ed25519Keypair {
    pub fn new() -> Self {
        // a.k.a Cryptographically secure pseudo-random number generator.
        let mut csprng = OsRng {};
        Ed25519Keypair {
            raw: ed25519_dalek::Keypair::generate(&mut csprng),
        }
    }

    pub fn construct_from_private_key(private_key: &Ed25519PrivateKey) -> Self {
        let sk: SecretKey = SecretKey::from_bytes(&(private_key.raw.to_bytes())).unwrap();
        let pk: PublicKey = (&sk).into();

        let keypair = ed25519_dalek::Keypair {
            secret: sk,
            public: pk,
        };

        Ed25519Keypair { raw: keypair }
    }
}

impl Drop for Ed25519Keypair {
    fn drop(&mut self) {
        println!("{:?} is being deallocated", self);
    }
}

#[no_mangle]
pub extern "C" fn c_ed25519_keypair_new() -> *mut Ed25519Keypair {
    let keypair = Ed25519Keypair::new();
    Box::into_raw(Box::new(keypair))
}

#[no_mangle]
pub extern "C" fn c_ed25519_keypair_destroy(keypair: *mut Ed25519Keypair) {
    let _ = unsafe { Box::from_raw(keypair) };
}

#[no_mangle]
pub unsafe extern "C" fn c_ed25519_keypair_get_private_key(
    keypair: *mut Ed25519Keypair,
) -> *mut Ed25519PrivateKey {
    let keypair = &mut *keypair;
    let private_key = keypair.get_private_key();
    Box::into_raw(Box::new(private_key))
}

#[no_mangle]
pub unsafe extern "C" fn c_ed25519_keypair_get_public_key(
    keypair: *mut Ed25519Keypair,
) -> *mut Ed25519PublicKey {
    let keypair = &mut *keypair;
    let public_key = keypair.get_public_key();
    Box::into_raw(Box::new(public_key))
}

#[no_mangle]
pub extern "C" fn c_ed25519_keypair_construct_from_private_key(
    private_key: &Ed25519PrivateKey,
) -> *mut Ed25519Keypair {
    let keypair = Ed25519Keypair::construct_from_private_key(&private_key);
    Box::into_raw(Box::new(keypair))
}
