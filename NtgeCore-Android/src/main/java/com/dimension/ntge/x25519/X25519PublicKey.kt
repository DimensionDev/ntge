package com.dimension.ntge.x25519

import com.dimension.ntge.Ntge

class X25519PublicKey internal constructor(
        internal val ptr: Long
) : AutoCloseable {

    override fun close() {
        Ntge.destroyX25519PublicKey(ptr)
    }
}