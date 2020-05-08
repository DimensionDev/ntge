package com.dimension.ntge

import androidx.benchmark.junit4.BenchmarkRule
import androidx.benchmark.junit4.measureRepeated
import androidx.test.ext.junit.runners.AndroidJUnit4
import com.dimension.ntge.ed25519.Ed25519PrivateKey
import com.dimension.ntge.ed25519.Ed25519PublicKey
import com.dimension.ntge.message.Decryptor
import com.dimension.ntge.message.Encryptor
import com.dimension.ntge.message.Message
import org.junit.Assert
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class MessageBenchmark {
    @get:Rule
    val benchmarkRule = BenchmarkRule()

    @Test
    fun benchmark_deserialize_message() = benchmarkRule.measureRepeated {
        Message.deserialize(message_to_dec)
    }

    @Test
    fun benchmark_serialize_message() {
        Message.deserialize(message_to_dec).let { message ->
            benchmarkRule.measureRepeated {
                message.serialize()
            }
        }
    }

    @Test
    fun benchmark_encrypt_message() {
        Ed25519PublicKey.deserialize(test_publicKey).let { ed25519PublicKey ->
            ed25519PublicKey.toX25519().let { x25519PublicKey ->
                Encryptor.new(x25519PublicKey).let { encryptor ->
                    benchmarkRule.measureRepeated {
                        encryptor.encryptPlaintext(message_to_enc)
                    }
                }
            }
        }
    }

    @Test
    fun benchmark_should_get_message_file_key()  {
        Message.deserialize(message_to_dec).let { message ->
            Decryptor.new(message).let { decryptor ->
                Ed25519PrivateKey.deserialize(hello_privateKey).let { ed25519PrivateKey ->
                    ed25519PrivateKey.toX25519().let { x25519PrivateKey ->
                        benchmarkRule.measureRepeated {
                            decryptor.getFileKey(x25519PrivateKey)
                        }
                    }
                }
            }
        }
    }

    @Test
    fun benchmark_should_verify_message_mac() {
        Message.deserialize(message_to_dec).let { message ->
            Decryptor.new(message).let { decryptor ->
                Ed25519PrivateKey.deserialize(hello_privateKey).let { ed25519PrivateKey ->
                    ed25519PrivateKey.toX25519().let { x25519PrivateKey ->
                        benchmarkRule.measureRepeated {
                            decryptor.getFileKey(x25519PrivateKey)
                        }
                    }
                }
            }
        }
    }

    @Test
    fun benchmark_decrypt_message() {
        Message.deserialize(message_to_dec).let { message ->
            Decryptor.new(message).let { decryptor ->
                Ed25519PrivateKey.deserialize(hello_privateKey).let { ed25519PrivateKey ->
                    ed25519PrivateKey.toX25519().let { x25519PrivateKey ->
                        decryptor.getFileKey(x25519PrivateKey).let { x25519FileKey ->
                            benchmarkRule.measureRepeated {
                                decryptor.decryptPayload(x25519FileKey)
                            }
                        }
                    }
                }
            }
        }
    }

    @Test
    fun benchmark_encrypt_and_decrypt() = benchmarkRule.measureRepeated {
        Ed25519PrivateKey.new().let { ed25519PrivateKey ->
            ed25519PrivateKey.toX25519().let { x25519PrivateKey ->
                ed25519PrivateKey.publicKey.toX25519().let { x25519PublicKey ->
                    Encryptor.new(x25519PublicKey).let { encryptor ->
                        encryptor.encryptPlaintext(message_to_enc).let { message ->
                            Assert.assertTrue(message.ptr != 0L)
                            val msgString = message.serialize()
                            Assert.assertTrue(msgString.isNotEmpty())
                            Message.deserialize(msgString).let { dec_message ->
                                Decryptor.new(dec_message).let { decryptor ->
                                    decryptor.getFileKey(x25519PrivateKey).let { x25519FileKey ->
                                        decryptor.decryptPayload(x25519FileKey)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    @Test
    fun benchmark_encrypt_message_with_signature() {
        Ed25519PublicKey.deserialize(test_publicKey).let { ed25519PublicKey ->
            ed25519PublicKey.toX25519().let { x25519PublicKey ->
                Encryptor.new(x25519PublicKey).let { encryptor ->
                    Ed25519PrivateKey.deserialize(test_privateKey).let { ed25519PrivateKey ->
                        benchmarkRule.measureRepeated {
                            encryptor.encryptPlaintext(message_to_enc, ed25519PrivateKey)
                        }
                    }
                }
            }
        }
    }
}