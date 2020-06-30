package com.dimension.ntge

object Base58 {
    fun encode(value: String) = Ntge.base58Encode(value)
    fun decode(value: String) = Ntge.base58Decode(value).toString(Charsets.UTF_8)
}