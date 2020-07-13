package com.dimension.ntge.ed25519

import com.dimension.ntge.Ntge
import com.dimension.ntge.x25519.X25519PublicKey

class Ed25519PublicKey internal constructor(
        internal val ptr: Long
) : AutoCloseable {

    companion object {
        fun deserialize(input: String): Ed25519PublicKey {
            return Ed25519PublicKey(Ntge.deserializeEd25519PublicKey(input))
        }
    }

    val keyId by lazy {
        Ntge.publicKeyKeyId(ptr)
    }

    fun serialize(): String {
        return Ntge.serializeEd25519PublicKey(ptr)
    }

    fun toX25519(): X25519PublicKey {
        return X25519PublicKey(Ntge.ed25519PublicKeyToX25519(ptr))
    }

    fun verify(message: ByteArray, signature: ByteArray) = Ntge.ed25519PublicKeyVerify(ptr, message, signature)

    override fun toString(): String {
        return serialize()
    }

    override fun close() {
        Ntge.destroyEd25519PublicKey(ptr)
    }
}

