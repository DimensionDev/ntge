use bech32::{self, ToBase32};

use super::{private::Ed25519PrivateKey, CURVE_NAME_ED25519};

#[derive(Debug)]
pub struct Ed25519PublicKey {
    pub raw: ed25519_dalek::PublicKey,
}

impl Ed25519PublicKey {
    fn serialize(&self) -> String {
        let data = self.raw.to_bytes().to_base32();
        let encoded = bech32::encode("pub", data).unwrap();
        encoded + "-" + CURVE_NAME_ED25519
    }
}

impl<'a> From<&'a Ed25519PrivateKey> for Ed25519PublicKey {
    fn from(private_key: &Ed25519PrivateKey) -> Ed25519PublicKey {
        Ed25519PublicKey {
            raw: (&private_key.raw).into(),
        }
    }
}
