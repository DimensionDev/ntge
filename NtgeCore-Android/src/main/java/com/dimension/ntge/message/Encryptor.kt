package com.dimension.ntge.message

import com.dimension.ntge.Ntge
import com.dimension.ntge.ed25519.Ed25519PrivateKey
import com.dimension.ntge.x25519.X25519PublicKey

class Encryptor internal constructor(
        internal val ptr: Long,
        private val keys_ptr: Long
) : AutoCloseable {

    companion object {
        fun new(vararg x25519PublicKey: X25519PublicKey): Encryptor {
            val keyPtr = Ntge.newArrayForX25519PublicKey()
            x25519PublicKey.forEach {
                Ntge.pushArrayX25519PublicKey(keyPtr, it.ptr)
            }
            return Encryptor(Ntge.newMessageEncryptor(keyPtr), keyPtr)
        }
    }

    fun encryptPlaintext(input: String, signatureKey: Ed25519PrivateKey? = null): Message {
        return Ntge.encryptPlaintext(input, ptr, signatureKey?.ptr
                ?: 0).let {
            Message(it)
        }
    }

    override fun close() {
        Ntge.destroyMessageEncryptor(ptr)
        Ntge.destroyArrayX25519PublicKey(keys_ptr)
    }

}