package com.dimension.ntge

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.Test
import org.junit.Assert.*
import org.junit.runner.RunWith
import com.dimension.ntge.ed25519.Ed25519PublicKey


@RunWith(AndroidJUnit4::class)
class Hmac256Test {
    @Test
    fun test() {
        Ed25519PublicKey.deserialize("pub1ryd8qreac4s2tz0ect98sn5hpjc7254qu6ea748urn3u2mxygmfqtx0hvq-Ed25519").use {
            val result = Hmac256.calculate(it, "Hello, World!");
            assertTrue(result == "45cf8a356f3cdebda7ccc08fdea82a7112f9ec14bae66f2e715e48ccd5ec2541")
        }
    }
}