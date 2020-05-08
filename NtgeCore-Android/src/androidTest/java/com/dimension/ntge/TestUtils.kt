package com.dimension.ntge

import org.junit.Assert.fail

const val test_publicKey = "pub1pacz8fthpmemnvmehnk6n6y2vcc367j65w8c04xfz9qulhyzw2vqvmy8sg-Ed25519"
const val test_privateKey = "pri15umhflv9mjwj7spxdjq4chzjacfgryf8hgak9727k0gl2px84kkssvgypc-Ed25519"
const val hello_publicKey = "pub1e35qxm0zr87tf05lss2sajt0wp3p8q4nak3lzw47aa80wglwm36q6zhth3-Ed25519"
const val hello_privateKey = "pri156qr745at67g28wsq50n6r5qzkcegunwqkkg53wnuprc8w5cpgnsvgl6lp-Ed25519"
const val message_to_enc = "hello😀!"
const val message_to_dec = "MsgBegin_ciXpWbV9LWtTiBCak4CATF92sMSx6QZSkFF3LhvCDYoKxthfs6kvFj9DFY7WvESsWL6ywC5JHowGr31xx3dNm47vRpBHxDNjyj83TeJDVLY6rtgAgphpYx6rkgNM7Rvg73vq8scfALTkKuUkkwDP8pKF8RTHSV4Uidfsa22GBrJ6medq5V2UPYcF3gEZY7mySEYdaLiiTpqQhKcFH8etNG3f4tGdCCDUNMu63jehSvJrHJRczuxB676hC4sGwnLD5u4KZ8UhP9hRsHpL45hzgTHT1NRSe3DqZ5FEXonnU6Qc4gtYS7L6y5doyrFsq6wdRCSRqFCLGQnPsVw8efzowk4zSJRdUQEuzEgK4zwFN7XuwHZ7hXnrRcg5ma5Gsszky13wPQ858QXHVVN6999a376rpoMseocPzTBrX8fufBWejscXjRgQZuTkyYfbZ9SLD34MumAkF7ZNYan7WA6wk85HXQnxTbDb4j8XGs6nsEKTciqXCfKt7LBnmt8XhmKXQDLxC48e9G8HjfJ9WhqU6xUSzD2pFfGoSW1S4sX4AWtMNBpzNCmh8wgprHpRru6G4jA3qDdJ44swzhZW23fzPZCfo4E4zjYu9PXP1eAa1PYo61R33wCshjV_EndMsg"


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