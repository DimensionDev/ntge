pub use ed25519_dalek::{self, Keypair, PublicKey, SecretKey};
use rand::rngs::OsRng;

use crate::{ed25519::private::Ed25519PrivateKey, ed25519::public::Ed25519PublicKey};

#[derive(Debug, Default)]
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
        let secret_key: &SecretKey = &private_key.raw;
        let keypair: Keypair = construct_from_secret_key(&secret_key);

        Ed25519Keypair { raw: keypair }
    }
}

impl Drop for Ed25519Keypair {
    fn drop(&mut self) {
        if cfg!(feature = "drop-log-enable") {
            println!("{:?} is being deallocated", self);
        }
    }
}

pub fn construct_from_secret_key(private_key: &SecretKey) -> Keypair {
    let sk: SecretKey = SecretKey::from_bytes(&(private_key.to_bytes())).unwrap();
    let pk: PublicKey = (&sk).into();

    Keypair {
        public: pk,
        secret: sk,
    }
}

#[no_mangle]
#[cfg(target_os = "ios")]
pub extern "C" fn c_ed25519_keypair_new() -> *mut Ed25519Keypair {
    let keypair = Ed25519Keypair::new();
    Box::into_raw(Box::new(keypair))
}

#[no_mangle]
#[cfg(target_os = "ios")]
pub unsafe extern "C" fn c_ed25519_keypair_destroy(keypair: *mut Ed25519Keypair) {
    let _ = Box::from_raw(keypair);
}

#[no_mangle]
#[cfg(target_os = "ios")]
pub unsafe extern "C" fn c_ed25519_keypair_get_private_key(
    keypair: *mut Ed25519Keypair,
) -> *mut Ed25519PrivateKey {
    let keypair = &mut *keypair;
    let private_key = keypair.get_private_key();
    Box::into_raw(Box::new(private_key))
}

#[no_mangle]
#[cfg(target_os = "ios")]
pub unsafe extern "C" fn c_ed25519_keypair_get_public_key(
    keypair: *mut Ed25519Keypair,
) -> *mut Ed25519PublicKey {
    let keypair = &mut *keypair;
    let public_key = keypair.get_public_key();
    Box::into_raw(Box::new(public_key))
}

#[no_mangle]
#[cfg(target_os = "ios")]
pub extern "C" fn c_ed25519_keypair_construct_from_private_key(
    private_key: &Ed25519PrivateKey,
) -> *mut Ed25519Keypair {
    let keypair = Ed25519Keypair::construct_from_private_key(&private_key);
    Box::into_raw(Box::new(keypair))
}

#[cfg(test)]
mod tests {
    use crate::ed25519;

    #[test]
    fn it_creates_a_keypair() {
        let keypair = ed25519::keypair::Ed25519Keypair::new();

        let serialized_private_key = keypair.get_private_key().serialize();
        println!("{}", serialized_private_key);

        let serialized_public_key = keypair.get_public_key().serialize();
        println!("{}", serialized_public_key);

        let deserialized_private_key =
            ed25519::private::Ed25519PrivateKey::deserialize(&serialized_private_key);
        assert_eq!(deserialized_private_key.is_ok(), true);
        assert_eq!(
            keypair.get_private_key().raw.to_bytes(),
            deserialized_private_key.unwrap().raw.to_bytes()
        );

        let deserialized_public_key =
            ed25519::public::Ed25519PublicKey::deserialize(&serialized_public_key);
        assert_eq!(deserialized_public_key.is_ok(), true);
        assert_eq!(
            keypair.get_public_key().raw.to_bytes(),
            deserialized_public_key.unwrap().raw.to_bytes()
        );
    }

    #[test]
    fn it_sign_and_verify_signature() {
        let message = b"Hello, World!";
        let keypair = ed25519::keypair::Ed25519Keypair::new();
        let private_key = keypair.get_private_key();
        let public_key = keypair.get_public_key();

        let signature = &private_key.sign(&message[..]);
        let verify_result = public_key.verify(&message[..], &signature[..]).is_ok();
        assert_eq!(verify_result, true);
    }
}
