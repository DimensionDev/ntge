#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;
    use self::jni::objects::{JClass, JString};
    use self::jni::JNIEnv;
    use jni::sys::{jboolean, jbyteArray, jstring, jlong};
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
        public_key: jlong,
    ) {
        let _ = Box::from_raw(public_key as *mut Ed25519PublicKey);
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_serializeEd25519PublicKey(
        _env: JNIEnv,
        _class: JClass,
        public_key: jlong,
    ) -> jstring {
        let public_key = public_key as *mut Ed25519PublicKey;
        let output = _env
            .new_string((*public_key).serialize())
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
        private_key: jlong,
    ) {
        let _ = Box::from_raw(private_key as *mut Ed25519PrivateKey);
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_getPublicKeyFromEd25519PrivateKey(
        _env: JNIEnv,
        _class: JClass,
        private_key: jlong,
    ) -> *mut Ed25519PublicKey {
        let private_key = private_key as *mut Ed25519PrivateKey;
        let public_key = (*private_key).get_public_key();
        Box::into_raw(Box::new(public_key))
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_serializeEd25519PrivateKey(
        _env: JNIEnv,
        _class: JClass,
        private_key: jlong,
    ) -> jstring {
        let private_key = private_key as *mut Ed25519PrivateKey;
        let output = _env
            .new_string((*private_key).serialize())
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
        keypair: jlong,
    ) {
        let _ = Box::from_raw(keypair as *mut Ed25519Keypair);
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_getPrivateKeyFromEd25519Keypair(
        _env: JNIEnv,
        _class: JClass,
        keypair: jlong,
    ) -> *mut Ed25519PrivateKey {
        let keypair = keypair as *mut Ed25519Keypair;
        let private_key = (*keypair).get_private_key();
        Box::into_raw(Box::new(private_key))
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_getPublicKeyFromEd25519Keypair(
        _env: JNIEnv,
        _class: JClass,
        keypair: jlong,
    ) -> *mut Ed25519PublicKey {
        let keypair = keypair as *mut Ed25519Keypair;
        let public_key = (*keypair).get_public_key();
        Box::into_raw(Box::new(public_key))
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_getEd25519KeypairFromPrivateKey(
        _env: JNIEnv,
        _class: JClass,
        private_key: jlong,
    ) -> *mut Ed25519Keypair {
        let private_key = private_key as *mut Ed25519PrivateKey;
        let keypair = Ed25519Keypair::construct_from_private_key(&*private_key);
        Box::into_raw(Box::new(keypair))
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_destroyX25519PrivateKey(
        _env: JNIEnv,
        _class: JClass,
        private_key: jlong,
    ) {
        let _ = Box::from_raw(private_key as *mut X25519PrivateKey);
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_destroyX25519PublicKey(
        _env: JNIEnv,
        _class: JClass,
        public_key: jlong,
    ) {
        let _ = Box::from_raw(public_key as *mut X25519PublicKey);
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_destroyX25519FileKey(
        _env: JNIEnv,
        _class: JClass,
        file_key: jlong,
    ) {
        let _ = Box::from_raw(file_key as *mut FileKey);
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_destroyMessage(
        _env: JNIEnv,
        _class: JClass,
        message: jlong,
    ) {
        let _ = Box::from_raw(message as *mut Message);
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_serializeMessage(
        _env: JNIEnv,
        _class: JClass,
        message: jlong,
    ) -> jstring {
        let message = message as *mut Message;
        match (*message).serialize_to_armor() {
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
        decryptor: jlong,
    ) {
        let _ = Box::from_raw(decryptor as *mut Decryptor);
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_newMessageDecryptor(
        _env: JNIEnv,
        _class: JClass,
        message: jlong,
    ) -> *mut Decryptor {
        let message = message as *mut Message;
        let decryptor = Decryptor::new(&*message);
        Box::into_raw(Box::new(decryptor))
    }
    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_messageDecryptorVerifyMessageMac(
        _env: JNIEnv,
        _class: JClass,
        decryptor: jlong,
        file_key: jlong,
    ) -> jboolean {
        let decryptor = decryptor as *mut Decryptor;
        let file_key = file_key as *mut FileKey;
        (*decryptor).verify_message_mac(&*file_key) as u8
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_messageDecryptorDecryptFileKey(
        _env: JNIEnv,
        _class: JClass,
        decryptor: jlong,
        private_key: jlong,
    ) -> *mut FileKey {
        let decryptor = decryptor as *mut Decryptor;
        let private_key = private_key as *mut X25519PrivateKey;
        match (*decryptor).decrypt_file_key(&*private_key) {
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
        decryptor: jlong,
        file_key: jlong,
    ) -> jbyteArray {
        let decryptor = decryptor as *mut Decryptor;
        let file_key = file_key as *mut FileKey;
        match (*decryptor).decrypt_payload(&*file_key) {
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
        message: jlong,
        public_key: jlong,
    ) -> jboolean {
        let message = message as *mut Message;
        let public_key = public_key as *mut Ed25519PublicKey;

        Decryptor::verify_signature(&*message, &*public_key) as u8
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_ed25519PublicKeyToX25519(
        _env: JNIEnv,
        _class: JClass,
        public_key: jlong,
    ) -> *mut X25519PublicKey {
        let ed25519_public_key = public_key as *mut Ed25519PublicKey;
        let x25519_public_key = X25519PublicKey {
            raw: ed25519_public_key_to_x25519(&(*ed25519_public_key).raw),
        };
        Box::into_raw(Box::new(x25519_public_key))
    }
    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_ed25519PrivateKeyToX25519(
        _env: JNIEnv,
        _class: JClass,
        private_key: jlong,
    ) -> *mut X25519PrivateKey {
        let ed25519_private_key = private_key as *mut Ed25519PrivateKey;
        let x25519_private_key = X25519PrivateKey {
            raw: ed25519_private_key_to_x25519(&(*ed25519_private_key).raw),
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
        public_keys: jlong,
    ) {
        let _ = Box::from_raw(public_keys as *mut Vec<X25519PublicKey>);
    }
    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_pushArrayX25519PublicKey(
        _env: JNIEnv,
        _class: JClass,
        array: jlong,
        element: jlong,
    ) {
        let array = array as *mut Vec<X25519PublicKey>;
        let element = element as *mut X25519PublicKey;
        (*array).push((*element).clone());
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_newMessageEncryptor(
        _env: JNIEnv,
        _class: JClass,
        x25519_public_keys: jlong,
    ) -> *mut Encryptor {
        let x25519_public_keys = x25519_public_keys as *mut Vec<X25519PublicKey>;
        let encryptor = Encryptor::new(&*x25519_public_keys);
        Box::into_raw(Box::new(encryptor))
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_destroyMessageEncryptor(
        _env: JNIEnv,
        _class: JClass,
        encryptor: jlong,
    ) {
        let _ = Box::from_raw(encryptor as *mut Encryptor);
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_dimension_ntge_Ntge_encryptPlaintext(
        _env: JNIEnv,
        _class: JClass,
        input: JString,
        encryptor: jlong,
        signature_key: jlong,
    ) -> *mut Message {
        let encoded: String = _env
            .get_string(input)
            .expect("Couldn't get java string!")
            .into();
        let data = encoded.as_bytes();
        let encryptor = encryptor as *mut Encryptor;
        let signature_key = signature_key as *mut Ed25519PrivateKey;
        let message = (*encryptor).encrypt(&data[..], signature_key.as_ref());
        Box::into_raw(Box::new(message))
    }
}
