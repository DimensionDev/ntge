use ntge_core::{ed25519, key_utils};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_validates_a_valid_keypair() {
        let keypair = ed25519::create_keypair();
        assert!(key_utils::keypair_validation(&keypair.secret, &keypair.public).is_ok());
    }

    #[test]
    fn it_validates_an_invalid_keypair() {
        let keypair1 = ed25519::create_keypair();
        let keypair2 = ed25519::create_keypair();

        assert!(!(key_utils::keypair_validation(&keypair1.secret, &keypair2.public).is_ok()));
    }

    #[test]
    fn it_converts_an_ed25519_public_key_to_x25519() {
        let keypair = ed25519::create_keypair();
        let pubkey = key_utils::ed25519_public_key_to_x25519(&keypair.public);

        // no need to verify an X25519 key
        assert_ne!(pubkey.as_bytes(), &[0; 32]);
    }

    #[test]
    fn it_converts_an_ed25519_private_key_to_x25519() {
        let keypair = ed25519::create_keypair();
        let private_key = key_utils::ed25519_private_key_to_x25519(&keypair.secret);

        assert_ne!(private_key.to_bytes(), [0; 32]);
    }

    #[test]
    fn it_conducts_diffie_hellman_on_two_ed25519_keypairs() {
        let keypair1 = ed25519::create_keypair();
        let private_key1 = key_utils::ed25519_private_key_to_x25519(&keypair1.secret);
        let public_key1 = key_utils::ed25519_public_key_to_x25519(&keypair1.public);

        let keypair2 = ed25519::create_keypair();
        let private_key2 = key_utils::ed25519_private_key_to_x25519(&keypair2.secret);
        let public_key2 = key_utils::ed25519_public_key_to_x25519(&keypair2.public);

        assert_eq!(
            private_key1.diffie_hellman(&public_key2).as_bytes(),
            private_key2.diffie_hellman(&public_key1).as_bytes(),
        );
    }
}
