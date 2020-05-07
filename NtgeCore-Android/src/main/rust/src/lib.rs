#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;
    use self::jni::objects::{JClass, JString};
    use self::jni::JNIEnv;
    use jni::sys::{jboolean, jbyteArray, jstring};
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

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_destroyEd25519PublicKey(
        _env: JNIEnv,
        _class: JClass,
        public_key: *mut Ed25519PublicKey,
    ) {
        let _ = Box::from_raw(public_key);
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_serializeEd25519PublicKey(
        _env: JNIEnv,
        _class: JClass,
        public_key: *mut Ed25519PublicKey,
    ) -> jstring {
        let public_key = &mut *public_key;
        let output = _env
            .new_string(public_key.serialize())
            .expect("Couldn't create java string!");
        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_deserializeEd25519PublicKey(
        _env: JNIEnv,
        _class: JClass,
        input: JString,
    ) -> *mut Ed25519PublicKey {
        let encoded: String = _env
            .get_string(input)
            .expect("Couldn't get java string!")
            .into();
        let public_key = Ed25519PublicKey::deserialize(&encoded);
        match public_key {
            Ok(key) => Box::into_raw(Box::new(key)),
            Err(_) => {
                let _ = _env.throw_new(
                    "com/dimension/ntge/NtgeException",
                    "Can not deserialize Ed25519PublicKey",
                );
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_dimension_ntge_Ntge_newEd25519PrivateKey(
        _env: JNIEnv,
        _class: JClass,
    ) -> *mut Ed25519PrivateKey {
        let private_key = Ed25519PrivateKey::new();
        Box::into_raw(Box::new(private_key))
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_destroyEd25519PrivateKey(
        _env: JNIEnv,
        _class: JClass,
        private_key: *mut Ed25519PrivateKey,
    ) {
        let _ = Box::from_raw(private_key);
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_getPublicKeyFromEd25519PrivateKey(
        _env: JNIEnv,
        _class: JClass,
        private_key: *mut Ed25519PrivateKey,
    ) -> *mut Ed25519PublicKey {
        let private_key = &mut *private_key;
        let public_key = private_key.get_public_key();
        Box::into_raw(Box::new(public_key))
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_serializeEd25519PrivateKey(
        _env: JNIEnv,
        _class: JClass,
        private_key: *mut Ed25519PrivateKey,
    ) -> jstring {
        let private_key = &mut *private_key;
        let output = _env
            .new_string(private_key.serialize())
            .expect("Couldn't create java string!");
        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_deserializeEd25519PrivateKey(
        _env: JNIEnv,
        _class: JClass,
        input: JString,
    ) -> *mut Ed25519PrivateKey {
        let encoded: String = _env
            .get_string(input)
            .expect("Couldn't get java string!")
            .into();
        let private_key = Ed25519PrivateKey::deserialize(&encoded);
        match private_key {
            Ok(key) => Box::into_raw(Box::new(key)),
            Err(_) => {
                let _ = _env.throw_new(
                    "com/dimension/ntge/NtgeException",
                    "Can not deserialize Ed25519PrivateKey",
                );
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_dimension_ntge_Ntge_newEd25519Keypair(
        _env: JNIEnv,
        _class: JClass,
    ) -> *mut Ed25519Keypair {
        let keypair = Ed25519Keypair::new();
        Box::into_raw(Box::new(keypair))
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_destroyEd25519Keypair(
        _env: JNIEnv,
        _class: JClass,
        keypair: *mut Ed25519Keypair,
    ) {
        let _ = Box::from_raw(keypair);
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_getPrivateKeyFromEd25519Keypair(
        _env: JNIEnv,
        _class: JClass,
        keypair: *mut Ed25519Keypair,
    ) -> *mut Ed25519PrivateKey {
        let keypair = &mut *keypair;
        let private_key = keypair.get_private_key();
        Box::into_raw(Box::new(private_key))
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_getPublicKeyFromEd25519Keypair(
        _env: JNIEnv,
        _class: JClass,
        keypair: *mut Ed25519Keypair,
    ) -> *mut Ed25519PublicKey {
        let keypair = &mut *keypair;
        let public_key = keypair.get_public_key();
        Box::into_raw(Box::new(public_key))
    }

    #[no_mangle]
    pub extern "system" fn Java_com_dimension_ntge_Ntge_getEd25519KeypairFromPrivateKey(
        _env: JNIEnv,
        _class: JClass,
        private_key: &Ed25519PrivateKey,
    ) -> *mut Ed25519Keypair {
        let keypair = Ed25519Keypair::construct_from_private_key(&private_key);
        Box::into_raw(Box::new(keypair))
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_destroyX25519PrivateKey(
        _env: JNIEnv,
        _class: JClass,
        private_key: *mut X25519PrivateKey,
    ) {
        let _ = Box::from_raw(private_key);
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_destroyX25519PublicKey(
        _env: JNIEnv,
        _class: JClass,
        public_key: *mut X25519PublicKey,
    ) {
        let _ = Box::from_raw(public_key);
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_destroyX25519FileKey(
        _env: JNIEnv,
        _class: JClass,
        file_key: *mut FileKey,
    ) {
        let _ = Box::from_raw(file_key);
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_destroyMessage(
        _env: JNIEnv,
        _class: JClass,
        message: *mut Message,
    ) {
        let _ = Box::from_raw(message);
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_serializeMessage(
        _env: JNIEnv,
        _class: JClass,
        message: *mut Message,
    ) -> jstring {
        let message = &mut *message;
        match message.serialize_to_armor() {
            Ok(text) => {
                let output = _env.new_string(text).expect("Couldn't create java string!");
                output.into_inner()
            }
            Err(_) => {
                let _ = _env.throw_new(
                    "com/dimension/ntge/NtgeException",
                    "Can not serialize message",
                );
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_deserializeMessage(
        _env: JNIEnv,
        _class: JClass,
        message: JString,
    ) -> *mut Message {
        let message: String = _env
            .get_string(message)
            .expect("Couldn't get java string!")
            .into();
        match Message::deserialize_from_armor(&message) {
            Ok(it) => Box::into_raw(Box::new(it)),
            Err(_) => {
                let _ = _env.throw_new(
                    "com/dimension/ntge/NtgeException",
                    "Can not deserialize message",
                );
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_destroyMessageDecryptor(
        _env: JNIEnv,
        _class: JClass,
        decryptor: *mut Decryptor,
    ) {
        let _ = Box::from_raw(decryptor);
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_newMessageDecryptor(
        _env: JNIEnv,
        _class: JClass,
        message: *mut Message,
    ) -> *mut Decryptor {
        let message = &mut *message;
        let decryptor = Decryptor::new(&message);
        Box::into_raw(Box::new(decryptor))
    }
    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_messageDecryptorVerifyMessageMac(
        _env: JNIEnv,
        _class: JClass,
        decryptor: *mut Decryptor,
        file_key: *mut FileKey,
    ) -> jboolean {
        let decryptor = &mut *decryptor;
        let file_key = &mut *file_key;
        decryptor.verify_message_mac(&file_key) as u8
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_messageDecryptorDecryptFileKey(
        _env: JNIEnv,
        _class: JClass,
        decryptor: *mut Decryptor,
        private_key: *mut X25519PrivateKey,
    ) -> *mut FileKey {
        let decryptor = &mut *decryptor;
        let private_key = &mut *private_key;
        match decryptor.decrypt_file_key(private_key) {
            Some(file_key) => Box::into_raw(Box::new(file_key)),
            None => {
                let _ = _env.throw_new("com/dimension/ntge/NtgeException", "Can not get fileKey");
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_messageDecryptorDecryptPayload(
        _env: JNIEnv,
        _class: JClass,
        decryptor: *mut Decryptor,
        file_key: *mut FileKey,
    ) -> jbyteArray {
        let decryptor = &mut *decryptor;
        let file_key = &mut *file_key;
        match decryptor.decrypt_payload(&file_key) {
            Some(bytes) => _env.byte_array_from_slice(&bytes).unwrap(),
            None => {
                let _ = _env.throw_new(
                    "com/dimension/ntge/NtgeException",
                    "Can not decrypt payload",
                );
                std::ptr::null_mut()
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_messageDecryptorVerifySignature(
        _env: JNIEnv,
        _class: JClass,
        message: *mut Message,
        public_key: *mut Ed25519PublicKey,
    ) -> jboolean {
        let message = &mut *message;
        let public_key = &mut *public_key;

        Decryptor::verify_signature(&message, &public_key) as u8
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_ed25519PublicKeyToX25519(
        _env: JNIEnv,
        _class: JClass,
        public_key: *mut Ed25519PublicKey,
    ) -> *mut X25519PublicKey {
        let ed25519_public_key = &mut *public_key;
        let x25519_public_key = X25519PublicKey {
            raw: ed25519_public_key_to_x25519(&ed25519_public_key.raw),
        };
        Box::into_raw(Box::new(x25519_public_key))
    }
    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_ed25519PrivateKeyToX25519(
        _env: JNIEnv,
        _class: JClass,
        private_key: *mut Ed25519PrivateKey,
    ) -> *mut X25519PrivateKey {
        let ed25519_private_key = &mut *private_key;
        let x25519_private_key = X25519PrivateKey {
            raw: ed25519_private_key_to_x25519(&ed25519_private_key.raw),
        };
        Box::into_raw(Box::new(x25519_private_key))
    }

    #[no_mangle]
    pub extern "system" fn Java_com_dimension_ntge_Ntge_newArrayForX25519PublicKey(
        _env: JNIEnv,
        _class: JClass,
    ) -> *mut Vec<X25519PublicKey> {
        let array: Vec<X25519PublicKey> = Vec::new();
        Box::into_raw(Box::new(array))
    }
    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_destroyArrayX25519PublicKey(
        _env: JNIEnv,
        _class: JClass,
        public_keys: *mut Vec<X25519PublicKey>,
    ) {
        let _ = Box::from_raw(public_keys);
    }
    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_pushArrayX25519PublicKey(
        _env: JNIEnv,
        _class: JClass,
        array: *mut Vec<X25519PublicKey>,
        element: *mut X25519PublicKey,
    ) {
        let array = &mut *array;
        let element = &mut *element;
        array.push((*element).clone());
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_newMessageEncryptor(
        _env: JNIEnv,
        _class: JClass,
        x25519_public_keys: *mut Vec<X25519PublicKey>,
    ) -> *mut Encryptor {
        let x25519_public_keys = &mut *x25519_public_keys;
        let encryptor = Encryptor::new(&x25519_public_keys);
        Box::into_raw(Box::new(encryptor))
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_destroyMessageEncryptor(
        _env: JNIEnv,
        _class: JClass,
        encryptor: *mut Encryptor,
    ) {
        let _ = Box::from_raw(encryptor);
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_encryptPlaintext(
        _env: JNIEnv,
        _class: JClass,
        input: JString,
        encryptor: *mut Encryptor,
        signature_key: *mut Ed25519PrivateKey,
    ) -> *mut Message {
        let encoded: String = _env
            .get_string(input)
            .expect("Couldn't get java string!")
            .into();
        let data = encoded.as_bytes();
        let encryptor = &mut *encryptor;
        let signature_key = signature_key.as_ref();
        let message = encryptor.encrypt(&data[..], signature_key);
        Box::into_raw(Box::new(message))
    }
}
