use bech32::{self, FromBase32, ToBase32};
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;

fn create_ed25519_keypair() -> Keypair {
    let mut csprng: OsRng = OsRng {};
    Keypair::generate(&mut csprng)
}

fn serialize_private_key(keypair: &Keypair) -> String {
    let data = keypair.secret.to_bytes().to_base32();
    let encoded = bech32::encode("pri", data).unwrap();
    encoded + "-Ed25519"
}

fn serialize_public_key(keypair: &Keypair) -> String {
    let data = keypair.public.to_bytes().to_base32();
    let encoded = bech32::encode("pub", data).unwrap();
    encoded + "-Ed25519"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_create_keypair() {
        let keypair = create_ed25519_keypair();

        let serialized_private_key = serialize_private_key(&keypair);
        println!("{}", serialized_private_key);

        let serialized_public_key = serialize_public_key(&keypair);
        println!("{}", serialized_public_key);
    }
}
