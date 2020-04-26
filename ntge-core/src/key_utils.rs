pub use crate::ed25519::*;
use curve25519_dalek::edwards::{CompressedEdwardsY, EdwardsPoint};
use ed25519_dalek::Keypair;
use ed25519_dalek::{ExpandedSecretKey, Signature};
use ed25519_dalek::{PublicKey, SecretKey};
use sha2::{Digest, Sha512};
use x25519_dalek::StaticSecret;

use super::error;
pub const CURVE_NAME_ED25519: &str = "Ed25519";

#[cfg(target_os = "ios")]
use crate::{
    ed25519::private::Ed25519PrivateKey, ed25519::public::Ed25519PublicKey,
    x25519::private::X25519PrivateKey, x25519::public::X25519PublicKey,
};

pub fn keypair_validation(
    private_key: &SecretKey,
    public_key: &PublicKey,
) -> Result<bool, error::CoreError> {
    let keypair: Keypair = keypair::construct_from_secret_key(private_key);
    if keypair.public.to_bytes() != (*public_key).to_bytes() {
        let core_error = error::CoreError::KeyInvalidError {
            name: CURVE_NAME_ED25519,
            reason: "The given public key and private key do not match.",
        };
        return Err(core_error);
    }

    let expanded_private: ExpandedSecretKey = ExpandedSecretKey::from(private_key);
    let message: &[u8] = b"Test";
    let signature: Signature = expanded_private.sign(message, public_key);

    match public_key.verify(message, &signature) {
        Ok(()) => Ok(true),
        Err(_) => {
            let core_error = error::CoreError::KeyInvalidError {
                name: CURVE_NAME_ED25519,
                reason: "The given public key and private key do not match.",
            };
            Err(core_error)
        }
    }
}

pub fn ed25519_public_key_to_x25519(public_key: &PublicKey) -> x25519_dalek::PublicKey {
    let public_key_bytes: [u8; 32] = (*public_key).to_bytes();
    let mut bits: [u8; 32] = [0u8; 32];
    bits.copy_from_slice(&public_key_bytes[..32]);
    let compressed = CompressedEdwardsY(bits);
    let edwardspoint: EdwardsPoint = compressed.decompress().unwrap();

    let x25519_public_key: x25519_dalek::PublicKey = edwardspoint.to_montgomery().to_bytes().into();
    x25519_public_key
}

// https://github.com/jedisct1/libsodium/blob/927dfe8e2eaa86160d3ba12a7e3258fbc322909c/src/libsodium/crypto_sign/ed25519/ref10/keypair.c#L69-L83
pub fn ed25519_private_key_to_x25519(private_key: &SecretKey) -> StaticSecret {
    let mut private_key_x25519 = [0; 32];
    private_key_x25519.copy_from_slice(&Sha512::digest(&(private_key.as_bytes())[0..32])[0..32]);

    // https://moderncrypto.org/mail-archive/curves/2014/000293.html
    private_key_x25519[0] &= 248;
    private_key_x25519[31] &= 127;
    private_key_x25519[31] |= 64;

    private_key_x25519.into()
}

#[no_mangle]
#[cfg(target_os = "ios")]
pub unsafe extern "C" fn c_key_utils_ed25519_public_key_to_x25519(
    public_key: *mut Ed25519PublicKey,
) -> *mut X25519PublicKey {
    let ed25519_public_key = &mut *public_key;
    let x25519_public_key = X25519PublicKey {
        raw: ed25519_public_key_to_x25519(&ed25519_public_key.raw),
    };
    Box::into_raw(Box::new(x25519_public_key))
}

#[no_mangle]
#[cfg(target_os = "ios")]
pub unsafe extern "C" fn c_key_utils_ed25519_private_key_to_x25519(
    private_key: *mut Ed25519PrivateKey,
) -> *mut X25519PrivateKey {
    let ed25519_private_key = &mut *private_key;
    let x25519_private_key = X25519PrivateKey {
        raw: ed25519_private_key_to_x25519(&ed25519_private_key.raw),
    };
    Box::into_raw(Box::new(x25519_private_key))
}

#[cfg(test)]
mod tests {
    use crate::{ed25519, key_utils};
    use rand::rngs::OsRng;
    use x25519_dalek::{EphemeralSecret, PublicKey, StaticSecret};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_validates_a_valid_keypair() {
        let keypair = ed25519::keypair::create_keypair();
        assert!(key_utils::keypair_validation(&keypair.secret, &keypair.public).is_ok());
    }

    #[test]
    fn it_validates_an_invalid_keypair() {
        let keypair1 = ed25519::keypair::create_keypair();
        let keypair2 = ed25519::keypair::create_keypair();

        assert!(!(key_utils::keypair_validation(&keypair1.secret, &keypair2.public).is_ok()));
    }

    #[test]
    fn it_converts_an_ed25519_public_key_to_x25519() {
        let keypair = ed25519::keypair::create_keypair();
        let pubkey = key_utils::ed25519_public_key_to_x25519(&keypair.public);

        // no need to verify an X25519 key
        assert_ne!(pubkey.as_bytes(), &[0; 32]);
    }

    #[test]
    fn it_converts_an_ed25519_private_key_to_x25519() {
        let keypair = ed25519::keypair::create_keypair();
        let private_key = key_utils::ed25519_private_key_to_x25519(&keypair.secret);

        assert_ne!(private_key.to_bytes(), [0; 32]);
    }

    #[test]
    fn it_conducts_diffie_hellman_on_two_ed25519_keypairs() {
        let alice = ed25519::keypair::create_keypair();
        let alice_private = key_utils::ed25519_private_key_to_x25519(&alice.secret);
        let alice_public = key_utils::ed25519_public_key_to_x25519(&alice.public);

        let bob = ed25519::keypair::create_keypair();
        let bob_private = key_utils::ed25519_private_key_to_x25519(&bob.secret);
        let bob_public = key_utils::ed25519_public_key_to_x25519(&bob.public);

        //Alice -> Bob
        let alice_private_ephemeral = EphemeralSecret::new(&mut OsRng);
        let alice_public_ephemeral = PublicKey::from(&alice_private_ephemeral);
        let alice_shared_secret1 = alice_private_ephemeral.diffie_hellman(&bob_public);
        let bob_private_static: StaticSecret = StaticSecret::from(bob_private.to_bytes());
        let bob_shared_secret1 = bob_private_static.diffie_hellman(&alice_public_ephemeral);
        assert_eq!(
            alice_shared_secret1.as_bytes(),
            bob_shared_secret1.as_bytes()
        );

        //Bob -> Alice
        let bob_private_ephemeral = EphemeralSecret::new(&mut OsRng);
        let bob_public_ephemeral = PublicKey::from(&bob_private_ephemeral);
        let bob_shared_secret2 = bob_private_ephemeral.diffie_hellman(&alice_public);
        let alice_private_static: StaticSecret = StaticSecret::from(alice_private.to_bytes());
        let alice_shared_secret2 = alice_private_static.diffie_hellman(&bob_public_ephemeral);
        assert_eq!(
            alice_shared_secret2.as_bytes(),
            bob_shared_secret2.as_bytes()
        );
    }
}
