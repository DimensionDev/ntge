package com.dimension.ntge


internal object Ntge {
    init {
        System.loadLibrary("ntgedroid")
    }
    external fun greeting(pattern: String): String
}
