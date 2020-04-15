use serde::{Deserialize, Serialize};
use serde_bytes;
use x25519_dalek::PublicKey;

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageRecipientHeader {
    pub key_type: String,
    #[serde(with = "serde_bytes")]
    pub ephemeral_public_key: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub encrypted_file_key: Vec<u8>, // 32 bytes
}

impl MessageRecipientHeader {
    pub fn get_ephemeral_public_key(&self) -> PublicKey {
        let mut key_bytes = [0; 32];
        key_bytes.copy_from_slice(&self.ephemeral_public_key[..32]);
        PublicKey::from(key_bytes)
    }
}
