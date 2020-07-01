package com.dimension.ntge

import androidx.benchmark.junit4.BenchmarkRule
import androidx.benchmark.junit4.measureRepeated
import androidx.test.ext.junit.runners.AndroidJUnit4
import com.dimension.ntge.ed25519.Ed25519Keypair
import com.dimension.ntge.ed25519.Ed25519PrivateKey
import com.dimension.ntge.ed25519.Ed25519PublicKey
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class KeyBenchmark {
    @get:Rule
    val benchmarkRule = BenchmarkRule()

    @Test
    fun benchmark_create_Ed25519Keypair() = benchmarkRule.measureRepeated {
        Ed25519Keypair.new()
    }

    @Test
    fun benchmark_create_Ed25519PrivateKey() = benchmarkRule.measureRepeated {
        Ed25519PrivateKey.new()
    }

    @Test
    fun benchmark_serialize_Ed25519PrivateKey() {
        Ed25519PrivateKey.new().use { privateKey ->
            benchmarkRule.measureRepeated {
                privateKey.serialize()
            }
        }
    }

    @Test
    fun benchmark_serialize_Ed25519Keypair() {
        Ed25519Keypair.new().use { keypair ->
            benchmarkRule.measureRepeated {
                keypair.publicKey.serialize()
                keypair.privateKey.serialize()
            }
        }
    }

    @Test
    fun benchmark_deserialize_Ed25519PublicKey() = benchmarkRule.measureRepeated {
        Ed25519PublicKey.deserialize(test_publicKey)
    }

    @Test
    fun benchmark_deserialize_Ed25519PrivateKey() = benchmarkRule.measureRepeated {
        Ed25519PrivateKey.deserialize(test_privateKey)
    }


    @Test
    fun benchmark_create_Ed25519KeyPair_from_Ed25519PrivateKey() {
        Ed25519PrivateKey.new().use { privateKey ->
            benchmarkRule.measureRepeated {
                Ed25519Keypair.fromPrivateKey(privateKey)
            }
        }
    }

    @Test
    fun benchmark_convert_Ed25519PublicKey_to_X25519PublicKey() {
        Ed25519PublicKey.deserialize(test_publicKey).use { ed25519PublicKey ->
            benchmarkRule.measureRepeated {
                ed25519PublicKey.toX25519()
            }
        }
    }

    @Test
    fun benchmark_convert_Ed25519PrivateKey_to_X25519PrivateKey() {
        Ed25519PrivateKey.new().use { ed25519PrivateKey ->
            benchmarkRule.measureRepeated {
                ed25519PrivateKey.toX25519()
            }
        }
    }
}