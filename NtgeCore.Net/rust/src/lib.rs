#[allow(non_snake_case)]
pub mod net_core {
    use ntge_core::ed25519::keypair::Ed25519Keypair;
    use ntge_core::ed25519::private::Ed25519PrivateKey;
    use ntge_core::ed25519::public::Ed25519PublicKey;
    use ntge_core::key_utils::{ed25519_private_key_to_x25519, ed25519_public_key_to_x25519};
    use ntge_core::message::decryptor::Decryptor;
    use ntge_core::message::encryptor::Encryptor;
    use ntge_core::message::Message;
    use ntge_core::x25519::filekey::FileKey;
    use ntge_core::x25519::private::X25519PrivateKey;
    use ntge_core::x25519::public::X25519PublicKey;
    use std::ffi::CStr;
    use std::ffi::CString;
    use std::os::raw::c_char;

    #[no_mangle]
    pub unsafe extern "C" fn free_string(s: *mut c_char) {
        if s.is_null() {
            return;
        }
        let _ = CString::from_raw(s);
    }

    #[no_mangle]
    pub unsafe extern "C" fn destroyEd25519PublicKey(public_key: *mut Ed25519PublicKey) {
        let _ = Box::from_raw(public_key);
    }

    #[no_mangle]
    pub unsafe extern "C" fn serializeEd25519PublicKey(
        public_key: *mut Ed25519PublicKey,
    ) -> *mut c_char {
        let value: String = (*public_key).serialize();
        CString::new(value).unwrap().into_raw()
    }

    #[no_mangle]
    pub unsafe extern "C" fn deserializeEd25519PublicKey(
        input: *const c_char,
    ) -> *mut Ed25519PublicKey {
        let encoded = std::str::from_utf8(CStr::from_ptr(input).to_bytes()).unwrap();
        let public_key = Ed25519PublicKey::deserialize(&encoded);
        match public_key {
            Ok(key) => Box::into_raw(Box::new(key)),
            Err(_) => std::ptr::null_mut(),
        }
    }

    #[no_mangle]
    pub extern "C" fn newEd25519PrivateKey() -> *mut Ed25519PrivateKey {
        let private_key = Ed25519PrivateKey::new();
        Box::into_raw(Box::new(private_key))
    }

    #[no_mangle]
    pub unsafe extern "C" fn destroyEd25519PrivateKey(private_key: *mut Ed25519PrivateKey) {
        let _ = Box::from_raw(private_key);
    }

    #[no_mangle]
    pub unsafe extern "C" fn getPublicKeyFromEd25519PrivateKey(
        private_key: *mut Ed25519PrivateKey,
    ) -> *mut Ed25519PublicKey {
        let public_key = (*private_key).get_public_key();
        Box::into_raw(Box::new(public_key))
    }

    #[no_mangle]
    pub unsafe extern "C" fn serializeEd25519PrivateKey(
        private_key: *mut Ed25519PrivateKey,
    ) -> *mut c_char {
        let value = (*private_key).serialize();
        CString::new(value).unwrap().into_raw()
    }

    #[no_mangle]
    pub unsafe extern "C" fn deserializeEd25519PrivateKey(
        input: *const c_char,
    ) -> *mut Ed25519PrivateKey {
        let encoded = std::str::from_utf8(CStr::from_ptr(input).to_bytes()).unwrap();
        let private_key = Ed25519PrivateKey::deserialize(&encoded);
        match private_key {
            Ok(key) => Box::into_raw(Box::new(key)),
            Err(_) => std::ptr::null_mut(),
        }
    }

    #[no_mangle]
    pub extern "C" fn newEd25519Keypair() -> *mut Ed25519Keypair {
        let keypair = Ed25519Keypair::new();
        Box::into_raw(Box::new(keypair))
    }

    #[no_mangle]
    pub unsafe extern "C" fn destroyEd25519Keypair(keypair: *mut Ed25519Keypair) {
        let _ = Box::from_raw(keypair);
    }

    #[no_mangle]
    pub unsafe extern "C" fn getPrivateKeyFromEd25519Keypair(
        keypair: *mut Ed25519Keypair,
    ) -> *mut Ed25519PrivateKey {
        let keypair = keypair;
        let private_key = (*keypair).get_private_key();
        Box::into_raw(Box::new(private_key))
    }

    #[no_mangle]
    pub unsafe extern "C" fn getPublicKeyFromEd25519Keypair(
        keypair: *mut Ed25519Keypair,
    ) -> *mut Ed25519PublicKey {
        let keypair = keypair;
        let public_key = (*keypair).get_public_key();
        Box::into_raw(Box::new(public_key))
    }

    #[no_mangle]
    pub unsafe extern "C" fn getEd25519KeypairFromPrivateKey(
        private_key: *mut Ed25519PrivateKey,
    ) -> *mut Ed25519Keypair {
        let private_key = private_key;
        let keypair = Ed25519Keypair::construct_from_private_key(&*private_key);
        Box::into_raw(Box::new(keypair))
    }

    #[no_mangle]
    pub unsafe extern "C" fn destroyX25519PrivateKey(private_key: *mut X25519PrivateKey) {
        let _ = Box::from_raw(private_key);
    }

    #[no_mangle]
    pub unsafe extern "C" fn destroyX25519PublicKey(public_key: *mut X25519PublicKey) {
        let _ = Box::from_raw(public_key);
    }

    #[no_mangle]
    pub unsafe extern "C" fn destroyX25519FileKey(file_key: *mut FileKey) {
        let _ = Box::from_raw(file_key);
    }

    #[no_mangle]
    pub unsafe extern "C" fn destroyMessage(message: *mut Message) {
        let _ = Box::from_raw(message);
    }

    #[no_mangle]
    pub unsafe extern "C" fn serializeMessage(message: *mut Message) -> *mut c_char {
        match (*message).serialize_to_armor() {
            Ok(text) => CString::new(text).unwrap().into_raw(),
            Err(_) => std::ptr::null_mut(),
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn deserializeMessage(message: *const c_char) -> *mut Message {
        let message = std::str::from_utf8(CStr::from_ptr(message).to_bytes()).unwrap();
        match Message::deserialize_from_armor(&message) {
            Ok(it) => Box::into_raw(Box::new(it)),
            Err(_) => std::ptr::null_mut(),
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn destroyMessageDecryptor(decryptor: *mut Decryptor) {
        let _ = Box::from_raw(decryptor);
    }

    #[no_mangle]
    pub unsafe extern "C" fn newMessageDecryptor(message: *mut Message) -> *mut Decryptor {
        let decryptor = Decryptor::new(&*message);
        Box::into_raw(Box::new(decryptor))
    }
    #[no_mangle]
    pub unsafe extern "C" fn messageDecryptorVerifyMessageMac(
        decryptor: *mut Decryptor,
        file_key: *mut FileKey,
    ) -> bool {
        (*decryptor).verify_message_mac(&*file_key)
    }

    #[no_mangle]
    pub unsafe extern "C" fn messageDecryptorDecryptFileKey(
        decryptor: *mut Decryptor,
        private_key: *mut X25519PrivateKey,
    ) -> *mut FileKey {
        match (*decryptor).decrypt_file_key(&*private_key) {
            Some(file_key) => Box::into_raw(Box::new(file_key)),
            None => std::ptr::null_mut(),
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn messageDecryptorDecryptPayload(
        decryptor: *mut Decryptor,
        file_key: *mut FileKey,
    ) -> *mut c_char {
        match (*decryptor).decrypt_payload(&*file_key) {
            Some(bytes) => match std::str::from_utf8(&bytes) {
                Ok(v) => CString::new(v).unwrap().into_raw(),
                Err(_) => std::ptr::null_mut(),
            },
            None => std::ptr::null_mut(),
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn messageDecryptorVerifySignature(
        message: *mut Message,
        public_key: *mut Ed25519PublicKey,
    ) -> bool {
        Decryptor::verify_signature(&*message, &*public_key)
    }

    #[no_mangle]
    pub unsafe extern "C" fn ed25519PublicKeyToX25519(
        public_key: *mut Ed25519PublicKey,
    ) -> *mut X25519PublicKey {
        let x25519_public_key = X25519PublicKey {
            raw: ed25519_public_key_to_x25519(&(*public_key).raw),
        };
        Box::into_raw(Box::new(x25519_public_key))
    }
    #[no_mangle]
    pub unsafe extern "C" fn ed25519PrivateKeyToX25519(
        private_key: *mut Ed25519PrivateKey,
    ) -> *mut X25519PrivateKey {
        let x25519_private_key = X25519PrivateKey {
            raw: ed25519_private_key_to_x25519(&(*private_key).raw),
        };
        Box::into_raw(Box::new(x25519_private_key))
    }

    #[no_mangle]
    pub extern "C" fn newArrayForX25519PublicKey() -> *mut Vec<X25519PublicKey> {
        let array: Vec<X25519PublicKey> = Vec::new();
        Box::into_raw(Box::new(array))
    }
    #[no_mangle]
    pub unsafe extern "C" fn destroyArrayX25519PublicKey(public_keys: *mut Vec<X25519PublicKey>) {
        let _ = Box::from_raw(public_keys);
    }

    #[no_mangle]
    pub unsafe extern "C" fn pushArrayX25519PublicKey(
        array: *mut Vec<X25519PublicKey>,
        element: *mut X25519PublicKey,
    ) {
        (*array).push((*element).clone());
    }

    #[no_mangle]
    pub unsafe extern "C" fn newMessageEncryptor(
        x25519_public_keys: *mut Vec<X25519PublicKey>,
    ) -> *mut Encryptor {
        let encryptor = Encryptor::new(&*x25519_public_keys);
        Box::into_raw(Box::new(encryptor))
    }

    #[no_mangle]
    pub unsafe extern "C" fn destroyMessageEncryptor(encryptor: *mut Encryptor) {
        let _ = Box::from_raw(encryptor);
    }

    #[no_mangle]
    pub unsafe extern "C" fn encryptPlaintext(
        input: *const c_char,
        encryptor: *mut Encryptor,
        signature_key: *mut Ed25519PrivateKey,
    ) -> *mut Message {
        let encoded = std::str::from_utf8(CStr::from_ptr(input).to_bytes()).unwrap();
        let data = encoded.as_bytes();
        let message = (*encryptor).encrypt(&data[..], signature_key.as_ref());
        Box::into_raw(Box::new(message))
    }

    #[no_mangle]
    pub unsafe extern "C" fn publicKeyKeyId(public_key: *mut Ed25519PublicKey) -> *mut c_char {
        let public_key = &mut *public_key;
        let key_id = public_key.key_id();
        CString::new(key_id).unwrap().into_raw()
    }

    #[no_mangle]
    pub unsafe extern "C" fn decryptMessageExtra(
        decryptor: *mut Decryptor,
        file_key: *mut FileKey,
    ) -> *mut c_char {
        let decryptor = &mut *decryptor;
        let file_key = &mut *file_key;
        match decryptor.decrypt_extra(&file_key) {
            Some(bytes) => match std::str::from_utf8(&bytes) {
                Ok(v) => CString::new(v).unwrap().into_raw(),
                Err(_) => std::ptr::null_mut(),
            },
            None => std::ptr::null_mut(),
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn encryptPlaintextWithExtra(
        encryptor: *mut Encryptor,
        plaintext_buffer: *const c_char,
        extra_plaintext_buffer: *const c_char,
        signature_key: *mut Ed25519PrivateKey,
    ) -> *mut Message {
        let encryptor = &mut *encryptor;
        let data = CStr::from_ptr(plaintext_buffer).to_bytes();
        let extra_data = CStr::from_ptr(extra_plaintext_buffer).to_bytes();
        let signature_key = signature_key.as_ref();
        let message = encryptor.encrypt_with_extra(&data[..], Some(&extra_data[..]), signature_key);
        Box::into_raw(Box::new(message))
    }

    #[no_mangle]
    pub unsafe extern "C" fn messageTimestamp(message: *mut Message) -> *mut c_char {
        let message = &mut *message;
        match &message.meta.timestamp {
            Some(text) => CString::new(text.clone()).unwrap().into_raw(),
            None => std::ptr::null_mut(),
        }
    }
}
