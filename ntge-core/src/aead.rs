use chacha20poly1305::aead::generic_array::GenericArray;
use chacha20poly1305::aead::{self, Aead, NewAead};
use chacha20poly1305::ChaCha20Poly1305;

pub(crate) fn aead_encrypt(key: &[u8; 32], plaintext: &[u8]) -> Vec<u8> {
    aead_encrypt_with_nonce(key, &[0; 12], plaintext)
}

pub(crate) fn aead_encrypt_with_nonce(
    key: &[u8; 32],
    nonce: &[u8; 12],
    plaintext: &[u8],
) -> Vec<u8> {
    let key = GenericArray::clone_from_slice(key);
    let aead = ChaCha20Poly1305::new(&key);
    let nonce = GenericArray::from_slice(nonce);
    aead.encrypt(nonce, plaintext.as_ref())
        .expect("we won't overflow the ChaCha20 block counter")
}

pub(crate) fn aead_decrypt(key: &[u8; 32], ciphertext: &[u8]) -> Result<Vec<u8>, aead::Error> {
    aead_decrypt_with_nonce(key, &[0; 12], ciphertext)
}

pub(crate) fn aead_decrypt_with_nonce(
    key: &[u8; 32],
    nonce: &[u8; 12],
    ciphertext: &[u8],
) -> Result<Vec<u8>, aead::Error> {
    let key = GenericArray::clone_from_slice(key);
    let aead = ChaCha20Poly1305::new(&key);
    let nonce = GenericArray::from_slice(nonce);
    aead.decrypt(nonce, ciphertext.as_ref())
}
