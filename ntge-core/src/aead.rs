use chacha20poly1305::aead::generic_array::GenericArray;
use chacha20poly1305::aead::{self, Aead, NewAead};
use chacha20poly1305::ChaCha20Poly1305;

pub(crate) fn aead_encrypt(key: &[u8; 32], plaintext: &[u8]) -> Vec<u8> {
    let key = GenericArray::clone_from_slice(key);
    let aead = ChaCha20Poly1305::new(key);
    let nonce = GenericArray::from_slice(&[0; 12]);
    aead.encrypt(nonce, plaintext.as_ref())
        .expect("we won't overflow the ChaCha20 block counter")
}

pub(crate) fn aead_decrypt(key: &[u8; 32], ciphertext: &[u8]) -> Result<Vec<u8>, aead::Error> {
    let key = GenericArray::clone_from_slice(key);
    let aead = ChaCha20Poly1305::new(key);
    let nonce = GenericArray::from_slice(&[0; 12]);
    aead.decrypt(nonce, ciphertext.as_ref())
}
