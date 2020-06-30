use hkdf::Hkdf;
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::ed25519::public::Ed25519PublicKey;

#[cfg(target_os = "ios")]
use crate::buffer::Buffer;

pub const MAC_KEY_LABEL: &[u8] = b"ntge-hmac-utils-mac-key";

pub fn hmac256_calculate_using(public_key: &Ed25519PublicKey, data: &[u8]) -> [u8; 32] {
    // create hmac key from public key
    let nonce = [0; 32];
    let mut mac_key = [0; 32];
    Hkdf::<Sha256>::new(Some(&nonce), &public_key.raw.to_bytes())
        .expand(MAC_KEY_LABEL, &mut mac_key)
        .expect("mac_key is the correct length");

    let mut hasher = Hmac::<Sha256>::new_varkey(&mac_key).expect("HMAC can take key of any size");
    hasher.input(&data);
    hasher.result().code().into()
}

#[no_mangle]
#[cfg(target_os = "ios")]
pub unsafe extern "C" fn c_hmac_utils_hmac256_calculate_using(
    public_key: *mut Ed25519PublicKey,
    data_buffer: Buffer,
) -> Buffer {
    let public_key = &mut *public_key;
    let data_bytes = data_buffer.to_bytes();
    let bytes = hmac256_calculate_using(&public_key, &data_bytes).to_vec();
    let slice = bytes.into_boxed_slice();
    let data = slice.as_ptr();
    let len = slice.len();
    std::mem::forget(slice);
    Buffer { data, len }
}

#[cfg(test)]
mod tests {

    use super::hmac256_calculate_using;
    use crate::ed25519;

    #[test]
    fn it_calculate_hmac() {
        let input = b"Hello, World!";
        let stub_public_key = ed25519::public::Ed25519PublicKey::deserialize(
            "pub1ryd8qreac4s2tz0ect98sn5hpjc7254qu6ea748urn3u2mxygmfqtx0hvq-Ed25519",
        )
        .unwrap();
        let mac = hmac256_calculate_using(&stub_public_key, &input[..]);
        assert_eq!(
            hex::encode(mac),
            "45cf8a356f3cdebda7ccc08fdea82a7112f9ec14bae66f2e715e48ccd5ec2541"
        );
    }
}
