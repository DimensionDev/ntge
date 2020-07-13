package com.dimension.ntge.ed25519

import com.dimension.ntge.Ntge
import com.dimension.ntge.x25519.X25519PrivateKey

class Ed25519PrivateKey internal constructor(
        internal val ptr: Long
) : AutoCloseable {

    companion object {
        fun new(): Ed25519PrivateKey {
            return Ed25519PrivateKey(Ntge.newEd25519PrivateKey())
        }

        fun deserialize(input: String): Ed25519PrivateKey {
            return Ed25519PrivateKey(Ntge.deserializeEd25519PrivateKey(input))
        }
    }

    val publicKey by lazy {
        Ed25519PublicKey(Ntge.getPublicKeyFromEd25519PrivateKey(ptr))
    }

    fun serialize(): String {
        return Ntge.serializeEd25519PrivateKey(ptr)
    }

    fun sign(message: ByteArray) = Ntge.ed25519PrivateKeySign(ptr, message);

    fun toX25519(): X25519PrivateKey {
        return X25519PrivateKey(Ntge.ed25519PrivateKeyToX25519(ptr))
    }

    override fun close() {
        Ntge.destroyEd25519PrivateKey(ptr)
    }
}