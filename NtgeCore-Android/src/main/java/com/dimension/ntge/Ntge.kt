package com.dimension.ntge

import android.annotation.TargetApi
import android.os.Build
import java.lang.Exception


internal object Ntge {
    init {
        System.loadLibrary("ntgedroid")
    }
    external fun destroyEd25519PublicKey(ptr: Long)
    external fun serializeEd25519PublicKey(ptr: Long): String
    external fun deserializeEd25519PublicKey(input: String): Long
    external fun newEd25519PrivateKey(): Long
    external fun destroyEd25519PrivateKey(ptr: Long)
    external fun getPublicKeyFromEd25519PrivateKey(ptr: Long): Long
    external fun serializeEd25519PrivateKey(ptr: Long): String
    external fun deserializeEd25519PrivateKey(input: String): Long
    external fun newEd25519Keypair(): Long
    external fun destroyEd25519Keypair(ptr: Long): Long
    external fun getPrivateKeyFromEd25519Keypair(ptr: Long): Long
    external fun getPublicKeyFromEd25519Keypair(ptr: Long): Long
    external fun getEd25519KeypairFromPrivateKey(ptr: Long): Long
    external fun destroyX25519PrivateKey(ptr: Long)
    external fun destroyX25519PublicKey(ptr: Long)
    external fun destroyX25519FileKey(ptr: Long)
    external fun destroyMessage(ptr: Long)
    external fun serializeMessage(ptr: Long): String
    external fun destroyMessageDecryptor(ptr: Long)
    external fun newMessageDecryptor(message_ptr: Long): Long
    external fun messageDecryptorVerifyMessageMac(decryptor_ptr: Long, file_key_ptr: Long): Boolean
    external fun messageDecryptorDecryptFileKey(decryptor_ptr: Long, private_key_ptr: Long): Long
    external fun messageDecryptorDecryptPayload(decryptor_ptr: Long, file_key_ptr: Long): ByteArray
    external fun messageDecryptorVerifySignature(message_ptr: Long, public_key_ptr: Long): Boolean
    external fun ed25519PublicKeyToX25519(ptr: Long): Long
    external fun ed25519PrivateKeyToX25519(ptr: Long): Long
    external fun array_new_for_x25519_public_key(): Long
    external fun array_destroy_x25519_public_key(ptr: Long)
    external fun array_push_x25519_public_key(array_ptr: Long, element_ptr: Long)
    external fun newMessageEncryptor(keys_ptr: Long): Long
    external fun destroyMessageEncryptor(ptr: Long)
    external fun encryptPlaintext(encryptor_ptr: Long, input: ByteArray, signature_key_ptr: Long): Long
}

class NtgeException : Exception {
    constructor() : super()
    constructor(message: String?) : super(message)
    constructor(message: String?, cause: Throwable?) : super(message, cause)
    constructor(cause: Throwable?) : super(cause)
    @TargetApi(Build.VERSION_CODES.N)
    constructor(message: String?, cause: Throwable?, enableSuppression: Boolean, writableStackTrace: Boolean) : super(message, cause, enableSuppression, writableStackTrace)
}