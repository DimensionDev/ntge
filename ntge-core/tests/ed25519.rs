use ntge_core::ed25519;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_create_keypair() {
        let keypair = ed25519::create_keypair();

        let serialized_private_key = ed25519::serialize_private_key(&keypair);
        println!("{}", serialized_private_key);

        let serialized_public_key = ed25519::serialize_public_key(&keypair);
        println!("{}", serialized_public_key);
    }
}
