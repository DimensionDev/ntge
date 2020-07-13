package com.dimension.ntge

import androidx.test.ext.junit.runners.AndroidJUnit4
import com.dimension.ntge.ed25519.Ed25519Keypair
import com.dimension.ntge.ed25519.Ed25519PrivateKey
import com.dimension.ntge.ed25519.Ed25519PublicKey
import org.junit.Assert.*
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class KeyTest {
    @Test
    fun it_create_Ed25519Keypair() {
        Ed25519Keypair.new().use { keypair ->
            assertTrue(keypair.ptr != 0L)
            assertTrue(keypair.publicKey.ptr != 0L)
            assertTrue(keypair.privateKey.ptr != 0L)
        }
    }

    @Test
    fun it_create_Ed25519PrivateKey() {
        Ed25519PrivateKey.new().use { privateKey ->
            assertTrue(privateKey.ptr != 0L)
        }
    }

    @Test
    fun it_serialize_Ed25519PrivateKey() {
        Ed25519PrivateKey.new().use { privateKey ->
            val result = privateKey.serialize()
            assertTrue(result.isNotEmpty())
        }
    }

    @Test
    fun it_serialize_Ed25519Keypair() {
        Ed25519Keypair.new().use { keypair ->
            val publicKey = keypair.publicKey.serialize()
            assertTrue(publicKey.isNotEmpty())
            val privateKey = keypair.privateKey.serialize()
            assertTrue(privateKey.isNotEmpty())
        }
    }

    @Test
    fun it_deserialize_Ed25519PublicKey() {
        Ed25519PublicKey.deserialize(test_publicKey).use { publicKey ->
            assertTrue(publicKey.ptr != 0L)
        }
    }

    @Test
    fun it_deserialize_Ed25519PrivateKey() {
        Ed25519PrivateKey.deserialize(test_privateKey).use { privateKey ->
            assertTrue(privateKey.ptr != 0L)
        }
    }

    @Test
    fun it_should_not_deserialize_Ed25519PublicKey() {
        assertThrows(NtgeException::class.java) {
            Ed25519PublicKey.deserialize("")
        }
        assertThrows(NtgeException::class.java) {
            Ed25519PublicKey.deserialize("hello!")
        }
        assertThrows(NtgeException::class.java) {
            Ed25519PublicKey.deserialize("😉😊😋😎🤣😄😅😗")
        }
        assertThrows(NtgeException::class.java) {
            Ed25519PublicKey.deserialize(test_privateKey)
        }
        assertThrows(NtgeException::class.java) {
            Ed25519PublicKey.deserialize("pri15umhflv9mjwj7spxdjq4chzjacfgryf8hgak9727k0gl2px84kkssvgypc-Ed25518")
        }
    }


    @Test
    fun it_should_not_deserialize_Ed25519PrivateKey() {
        assertThrows(NtgeException::class.java) {
            Ed25519PrivateKey.deserialize("")
        }
        assertThrows(NtgeException::class.java) {
            Ed25519PrivateKey.deserialize("hello!")
        }
        assertThrows(NtgeException::class.java) {
            Ed25519PrivateKey.deserialize("😉😊😋😎🤣😄😅😗")
        }
        assertThrows(NtgeException::class.java) {
            Ed25519PrivateKey.deserialize(test_publicKey)
        }
        assertThrows(NtgeException::class.java) {
            Ed25519PrivateKey.deserialize("pub1pacz8fthpmemnvmehnk6n6y2vcc367j65w8c04xfz9qulhyzw2vqvmy8sg-Ed25518")
        }
    }

    @Test
    fun it_create_Ed25519KeyPair_from_Ed25519PrivateKey() {
        Ed25519PrivateKey.new().use { privateKey ->
            Ed25519Keypair.fromPrivateKey(privateKey).use { keypair ->
                assertTrue(keypair.ptr != 0L)
            }
        }
    }

    @Test
    fun it_convert_Ed25519PublicKey_to_X25519PublicKey() {
        Ed25519PublicKey.deserialize(test_publicKey).use { ed25519PublicKey ->
            ed25519PublicKey.toX25519().use { x25519PublicKey ->
                assertTrue(x25519PublicKey.ptr != 0L)
            }
        }
    }

    @Test
    fun it_convert_Ed25519PrivateKey_to_X25519PrivateKey() {
        Ed25519PrivateKey.new().use { ed25519PrivateKey ->
            ed25519PrivateKey.toX25519().use { x25519PrivateKey ->
                assertTrue(x25519PrivateKey.ptr != 0L)
            }
        }
    }

    @Test
    fun it_get_Ed25519PublicKey_KeyId() {
        Ed25519PublicKey.deserialize(test_publicKey).use { publicKey ->
            assertFalse(publicKey.keyId.isNullOrEmpty())
        }
    }

    @Test
    fun it_sign_and_verify_signature() {
        val message = "Hello, World!".toByteArray(Charsets.UTF_8)
        Ed25519Keypair.new().use { keypair ->
            val signature = keypair.privateKey.sign(message);
            assertTrue(signature.any())
            assertTrue(keypair.publicKey.verify(message, signature))
        }
    }
}

