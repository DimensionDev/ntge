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
        assert!(key_utils::keypair_validation(&keypair.secret, &keypair.public));
    }

    #[test]
    fn it_validates_an_invalid_keypair() {
        let keypair1 = ed25519::create_keypair();
        let keypair2 = ed25519::create_keypair();

        assert!(!(key_utils::keypair_validation(&keypair1.secret, &keypair2.public)));
    }
    
    #[test]
    fn it_converts_an_ed25519_public_key_to_x25519() {
        let keypair = ed25519::create_keypair();
        let pubkey = key_utils::ed25519_to_x25519(&keypair.public);
    }
}
