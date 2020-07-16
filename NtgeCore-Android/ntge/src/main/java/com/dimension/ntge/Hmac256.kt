package com.dimension.ntge

import com.dimension.ntge.ed25519.Ed25519PublicKey

object Hmac256 {
    fun calculate(publicKey: Ed25519PublicKey, value: ByteArray) = Ntge.hmac256Calculate(publicKey.ptr, value);
}