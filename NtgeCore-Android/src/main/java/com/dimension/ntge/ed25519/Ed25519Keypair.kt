package com.dimension.ntge.ed25519

import com.dimension.ntge.Ntge

class Ed25519Keypair internal constructor(
        internal val ptr: Long
) : AutoCloseable {

    companion object {
        fun new(): Ed25519Keypair {
            return Ed25519Keypair(Ntge.newEd25519Keypair())
        }
        fun fromPrivateKey(privateKey: Ed25519PrivateKey): Ed25519Keypair {
            return Ed25519Keypair(Ntge.getEd25519KeypairFromPrivateKey(privateKey.ptr))
        }
    }

    val privateKey = Ed25519PrivateKey(Ntge.getPrivateKeyFromEd25519Keypair(ptr))
    val publicKey = Ed25519PublicKey(Ntge.getPublicKeyFromEd25519Keypair(ptr))

    override fun close() {
        Ntge.destroyEd25519Keypair(ptr)
        privateKey.close()
        publicKey.close()
    }
}