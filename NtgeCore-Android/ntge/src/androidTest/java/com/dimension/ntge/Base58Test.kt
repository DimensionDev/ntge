package com.dimension.ntge

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.Test
import org.junit.Assert.*
import org.junit.runner.RunWith


@RunWith(AndroidJUnit4::class)
class Base58Test {
    @Test
    fun it_encode() {
        assertTrue(Base58.encode("Hello, World!") == "D7LMXYjYZ7cDaGe8bS")
    }
    
    @Test
    fun it_decode() {
        assertTrue(Base58.decode("D7LMXYjYZ7cDaGe8bS") == "Hello, World!")
    }
}