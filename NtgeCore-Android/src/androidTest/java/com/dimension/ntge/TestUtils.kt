package com.dimension.ntge

import org.junit.Assert.fail

const val test_publicKey = "pub1pacz8fthpmemnvmehnk6n6y2vcc367j65w8c04xfz9qulhyzw2vqvmy8sg-Ed25519"
const val test_privateKey = "pri15umhflv9mjwj7spxdjq4chzjacfgryf8hgak9727k0gl2px84kkssvgypc-Ed25519"
const val hello_publicKey = "pub1e35qxm0zr87tf05lss2sajt0wp3p8q4nak3lzw47aa80wglwm36q6zhth3-Ed25519"
const val hello_privateKey = "pri156qr745at67g28wsq50n6r5qzkcegunwqkkg53wnuprc8w5cpgnsvgl6lp-Ed25519"
const val message_to_enc = "hello😀!"
const val message_to_dec = "MsgBeginIIIIIRrJXG8oAk4L9EYvjm3pEmTWTB9jaqLFRaHUqDY6kz6xad2oqrSV7o6TVqkrP1kHiqCNHVFaiPVLjBCAwPTagSZ69FcEskq6mnA4ZzDDb1DZeBa6Z3WVAmJaqj8ZRSd8YSB5k9QGXZrUMJBX94iQ86KEAoWF6URFy8Vryr4wV82kjYnSDQKJShthYCxab7PWNwt5cs7bWpoqsFisn33n4NdCeUAZZ4MGdnqFGWtgBfcQKHWNHRj4UvZvdGYpaRatGDtmGoKX133MRmV3FmIIIIIEndMsg"


@Throws(Exception::class)
fun <T : Exception?> assertThrows(
        expected: Class<T>,
        codeUnderTest: () -> Unit): T? {
    var result: T? = null
    try {
        codeUnderTest.invoke()
        fail("Expecting exception but none was thrown.")
    } catch (actual: Exception) {
        result = if (expected.isInstance(actual)) {
            expected.cast(actual)
        } else {
            throw actual
        }
    }
    return result
}