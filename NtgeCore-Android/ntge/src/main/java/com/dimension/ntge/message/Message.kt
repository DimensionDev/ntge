package com.dimension.ntge.message

import com.dimension.ntge.Ntge
import com.dimension.ntge.ed25519.Ed25519PublicKey

class Message internal constructor(
        internal val ptr: Long
) : AutoCloseable {

    companion object {
        fun deserialize(input: String): Message {
            return Message(Ntge.deserializeMessage(input))
        }
    }

    val timestamp: String
        get() = Ntge.messageTimestamp(ptr)

    fun verifySignature(publicKey: Ed25519PublicKey): Boolean {
        return Ntge.messageDecryptorVerifySignature(ptr, publicKey.ptr)
    }

    fun serialize(): String {
        return Ntge.serializeMessage(ptr)
    }

    override fun toString(): String {
        return serialize()
    }

    override fun close() {
        Ntge.destroyMessage(ptr)
    }
}

