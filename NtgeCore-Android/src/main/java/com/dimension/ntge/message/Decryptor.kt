package com.dimension.ntge.message

import com.dimension.ntge.Ntge
import com.dimension.ntge.x25519.X25519FileKey
import com.dimension.ntge.x25519.X25519PrivateKey

class Decryptor internal constructor(
        internal val ptr: Long
) : AutoCloseable {

    companion object {
        fun new(message: Message): Decryptor {
            return Decryptor(Ntge.newMessageDecryptor(message.ptr))
        }
    }

    fun verifyMessageMac(fileKey: X25519FileKey): Boolean {
        return Ntge.messageDecryptorVerifyMessageMac(ptr, fileKey.ptr)
    }

    fun getFileKey(privateKey: X25519PrivateKey): X25519FileKey {
        return X25519FileKey(Ntge.messageDecryptorDecryptFileKey(ptr, privateKey.ptr))
    }

    fun decryptPayload(fileKey: X25519FileKey): String {
        return Ntge.messageDecryptorDecryptPayload(ptr, fileKey.ptr).toString(Charsets.UTF_8)
    }

    fun decryptPayloadExtra(fileKey: X25519FileKey): String {
        return Ntge.decryptMessageExtra(ptr, fileKey.ptr).toString(Charsets.UTF_8)
    }

    override fun close() {
        Ntge.destroyMessageDecryptor(ptr)
    }
}