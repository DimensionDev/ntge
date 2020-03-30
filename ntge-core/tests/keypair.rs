use ntge_core::keypair;
use ntge_core::keypair::{KeyPairType, Serializable};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_create_keypair() {
        let keypair: ed25519_dalek::Keypair = keypair::generate_keypair(keypair::KeyAlgo::Ed25519);
        let private_key = keypair.private_key();
        let public_key = keypair.public_key();
        let priv_key_text = private_key.serialize();
        let pub_key_text = public_key.serialize();
        assert_eq!(true, priv_key_text.len() > 0);
        assert_eq!(true, pub_key_text.len() > 0);
    }
}