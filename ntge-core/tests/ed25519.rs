use ntge_core::ed25519;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_creates_a_keypair() {
        let keypair = ed25519::create_keypair();

        let serialized_private_key = ed25519::serialize_private_key(&(keypair.secret));
        println!("{}", serialized_private_key);

        let serialized_public_key = ed25519::serialize_public_key(&(keypair.public));
        println!("{}", serialized_public_key);

        let deserialized_private_key = ed25519::deserialize_private_key(&serialized_private_key);
        assert_eq!(deserialized_private_key.is_ok(), true);
        assert_eq!(
            keypair.secret.to_bytes(),
            deserialized_private_key.unwrap().to_bytes()
        );

        let deserialized_public_key = ed25519::deserialize_public_key(&serialized_public_key);
        assert_eq!(deserialized_public_key.is_ok(), true);
        assert_eq!(
            keypair.public.to_bytes(),
            deserialized_public_key.unwrap().to_bytes()
        );
    }

    #[test]
    fn it_deserializes_a_private_key() {
        let encoded_private_key =
            "pri1kq9sn9nyutfwsrauz2akl0d0qxzu38dnes6q47x6tnaf57ad7xnsg2fq6l-Ed25519";
        let deserialized_private_key = ed25519::deserialize_private_key(&encoded_private_key);
        assert_eq!(true, deserialized_private_key.is_ok());
    }

    #[test]
    fn it_deserializes_a_public_key() {
        let encoded_public_key =
            "pub1w0yqh0eple0cpeqc0es7um553e6pfmyuam6x6cu3vaq602d7v6msnal7n5-Ed25519";
        let deserialized_public_key = ed25519::deserialize_public_key(&encoded_public_key);
        assert_eq!(true, deserialized_public_key.is_ok());
    }

    #[test]
    fn it_signs_a_message_and_self_verify() {
        let message = "TEST";
        let keypair = ed25519::create_keypair();

        let signature = ed25519::sign(&keypair.secret, message.as_bytes());
        let result = ed25519::verify(&keypair.public, message.as_bytes(), &signature);
        assert!(result.is_ok());
    }
}
