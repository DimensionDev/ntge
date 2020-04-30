package com.dimension.ntge.message

import com.dimension.ntge.Ntge
import com.dimension.ntge.ed25519.Ed25519PrivateKey
import com.dimension.ntge.x25519.X25519PublicKey

class Encryptor internal constructor(
        internal val ptr: Long,
        private val keys_ptr: Long,
        val keys: List<X25519PublicKey>
) : AutoCloseable {

    companion object {
        fun new(vararg x25519PublicKey: X25519PublicKey): Encryptor {
            val keyPtr = Ntge.array_new_for_x25519_public_key()
            x25519PublicKey.forEach {
                Ntge.array_push_x25519_public_key(keyPtr, it.ptr)
            }
            return Encryptor(Ntge.newMessageEncryptor(keyPtr), keyPtr, x25519PublicKey.toList())
        }
    }

    fun encryptPlaintext(input: String, signatureKey: Ed25519PrivateKey? = null) {
        Ntge.encryptPlaintext(ptr, input.toByteArray(), signatureKey?.ptr
                ?: 0)
    }

    override fun close() {
        Ntge.destroyMessageEncryptor(ptr)
        Ntge.array_destroy_x25519_public_key(keys_ptr)
        keys.forEach {
            it.close()
        }
    }

}