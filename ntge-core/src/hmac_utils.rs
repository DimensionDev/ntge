use hkdf::Hkdf;
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::ed25519::public::Ed25519PublicKey;

#[cfg(target_os = "ios")]
use crate::buffer::Buffer;

#[allow(dead_code)]
pub const MAC_KEY_LABEL: &[u8] = b"ntge-hmac-utils-mac-key";

#[derive(Debug, Clone)]
pub struct Hmac256 {
    hmac: Hmac<Sha256>,
}

impl Hmac256 {
    #[allow(dead_code)]
    pub(crate) fn new(public_key: &Ed25519PublicKey) -> Self {
        // create hmac key from file key
        let nonce = [0; 32];
        let mut mac_key = [0; 32];
        Hkdf::<Sha256>::new(Some(&nonce), &public_key.raw.to_bytes())
            .expand(MAC_KEY_LABEL, &mut mac_key)
            .expect("mac_key is the correct length");

        Hmac256 {
            hmac: Hmac::<Sha256>::new_varkey(&mac_key).expect("HMAC can take key of any size"),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn input(&mut self, data: &[u8]) {
        self.hmac.input(&data);
    }

    #[allow(dead_code)]
    pub(crate) fn reset(&mut self) {
        self.hmac.reset();
    }

    #[allow(dead_code)]
    // re-entry forbidden. only accept clone or move
    pub(crate) fn finish(self) -> [u8; 32] {
        self.hmac.result().code().into()
    }
}

#[no_mangle]
#[cfg(target_os = "ios")]
pub unsafe extern "C" fn c_hmac_utils_hmac256_new(
    public_key: *mut Ed25519PublicKey,
) -> *mut Hmac256 {
    let public_key = &mut *public_key;
    let hmac256 = Hmac256::new(&public_key);
    Box::into_raw(Box::new(hmac256))
}

#[no_mangle]
#[cfg(target_os = "ios")]
pub unsafe extern "C" fn c_hmac_utils_hmac256_destory(hmac256: *mut Hmac256) {
    let _ = Box::from_raw(hmac256);
}

#[no_mangle]
#[cfg(target_os = "ios")]
pub unsafe extern "C" fn c_hmac_utils_hmac_input(hmac256: *mut Hmac256, data_buffer: Buffer) {
    let hmac256 = &mut *hmac256;
    let input_data = data_buffer.to_bytes();
    hmac256.input(&input_data);
}

#[no_mangle]
#[cfg(target_os = "ios")]
pub unsafe extern "C" fn c_hmac_utils_hmac_reset(hmac256: *mut Hmac256) {
    let hmac256 = &mut *hmac256;
    hmac256.reset();
}

#[no_mangle]
#[cfg(target_os = "ios")]
pub unsafe extern "C" fn c_hmac_utils_hmac_finish(hmac256: *mut Hmac256) -> Buffer {
    let hmac256 = &mut *hmac256;
    let bytes = hmac256.finish().to_vec();
    let slice = bytes.into_boxed_slice();
    let data = slice.as_ptr();
    let len = slice.len();
    std::mem::forget(slice);
    Buffer { data, len }
}
