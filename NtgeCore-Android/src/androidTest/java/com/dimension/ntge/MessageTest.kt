package com.dimension.ntge

import androidx.test.ext.junit.runners.AndroidJUnit4
import com.dimension.ntge.ed25519.Ed25519PrivateKey
import com.dimension.ntge.ed25519.Ed25519PublicKey
import com.dimension.ntge.message.Decryptor
import com.dimension.ntge.message.Encryptor
import com.dimension.ntge.message.Message
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class MessageTest {
    @Test
    fun it_deserialize_message() {
        Message.deserialize(message_to_dec).use { message ->
            assertTrue(message.ptr != 0L)
        }
    }

    @Test
    fun it_serialize_message() {
        Message.deserialize(message_to_dec).use { message ->
            val result = message.serialize()
            assertTrue(result.isNotEmpty())
        }
    }

    @Test
    fun it_encrypt_message() {
        Ed25519PublicKey.deserialize(test_publicKey).use { ed25519PublicKey ->
            ed25519PublicKey.toX25519().use { x25519PublicKey ->
                Encryptor.new(x25519PublicKey).use { encryptor ->
                    encryptor.encryptPlaintext(message_to_enc).use { message ->
                        assertTrue(message.ptr != 0L)
                    }
                }
            }
        }
    }

    @Test
    fun it_should_get_message_file_key() {
        Message.deserialize(message_to_dec).use { message ->
            Decryptor.new(message).use { decryptor ->
                Ed25519PrivateKey.deserialize(test_privateKey).use { ed25519PrivateKey ->
                    ed25519PrivateKey.toX25519().use { x25519PrivateKey ->
                        decryptor.getFileKey(x25519PrivateKey).use { x25519FileKey ->
                            assertTrue(x25519FileKey.ptr != 0L)
                        }
                    }
                }
            }
        }
    }

    @Test
    fun it_should_verify_message_mac() {
        Message.deserialize(message_to_dec).use { message ->
            Decryptor.new(message).use { decryptor ->
                Ed25519PrivateKey.deserialize(test_privateKey).use { ed25519PrivateKey ->
                    ed25519PrivateKey.toX25519().use { x25519PrivateKey ->
                        decryptor.getFileKey(x25519PrivateKey).use { x25519FileKey ->
                            assertTrue(decryptor.verifyMessageMac(x25519FileKey))
                        }
                    }
                }
            }
        }
    }

    @Test
    fun it_decrypt_message() {
        Message.deserialize(message_to_dec).use { message ->
            Decryptor.new(message).use { decryptor ->
                Ed25519PrivateKey.deserialize(test_privateKey).use { ed25519PrivateKey ->
                    ed25519PrivateKey.toX25519().use { x25519PrivateKey ->
                        decryptor.getFileKey(x25519PrivateKey).use { x25519FileKey ->
                            val result = decryptor.decryptPayload(x25519FileKey)
                            assertTrue(result == message_to_enc)
                        }
                    }
                }
            }
        }
    }

    @Test
    fun it_encrypt_and_decrypt() {
        Ed25519PrivateKey.new().use { ed25519PrivateKey ->
            ed25519PrivateKey.toX25519().use { x25519PrivateKey ->
                ed25519PrivateKey.publicKey.toX25519().use { x25519PublicKey ->
                    Encryptor.new(x25519PublicKey).use { encryptor ->
                        encryptor.encryptPlaintext(message_to_enc).use { message ->
                            assertTrue(message.ptr != 0L)
                            Decryptor.new(message).use { decryptor ->
                                decryptor.getFileKey(x25519PrivateKey).use { x25519FileKey ->
                                    val result = decryptor.decryptPayload(x25519FileKey)
                                    assertTrue(result == message_to_enc)
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    @Test
    fun it_encrypt_message_with_signature() {
        Ed25519PublicKey.deserialize(test_publicKey).use { ed25519PublicKey ->
            ed25519PublicKey.toX25519().use { x25519PublicKey ->
                Encryptor.new(x25519PublicKey).use { encryptor ->
                    Ed25519PrivateKey.deserialize(test_privateKey).use { ed25519PrivateKey ->
                        encryptor.encryptPlaintext(message_to_enc, ed25519PrivateKey).use { message ->
                            assertTrue(message.ptr != 0L)
                        }
                    }
                }
            }
        }
    }

    @Test
    fun it_verify_message_signature() {
        Message.deserialize(message_with_sign).use { message ->
            Ed25519PrivateKey.deserialize(test_privateKey).use { ed25519PrivateKey ->
                assertTrue(message.verifySignature(ed25519PrivateKey.publicKey))
                Ed25519PrivateKey.new().use {
                    assertFalse(message.verifySignature(it.publicKey))
                }
            }
        }
    }

    @Test
    fun it_decrypt_message_with_signature() {
        Message.deserialize(message_with_sign).use { message ->
            Decryptor.new(message).use { decryptor ->
                Ed25519PrivateKey.deserialize(test_privateKey).use { ed25519PrivateKey ->
                    ed25519PrivateKey.toX25519().use { x25519PrivateKey ->
                        decryptor.getFileKey(x25519PrivateKey).use { x25519FileKey ->
                            val result = decryptor.decryptPayload(x25519FileKey)
                            assertTrue(result == message_to_enc)
                        }
                    }
                }
            }
        }
    }
}