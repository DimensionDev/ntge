package com.dimension.ntge

import org.junit.Assert.fail

val test_publicKey = "pub1pacz8fthpmemnvmehnk6n6y2vcc367j65w8c04xfz9qulhyzw2vqvmy8sg-Ed25519"
val test_privateKey = "pri15umhflv9mjwj7spxdjq4chzjacfgryf8hgak9727k0gl2px84kkssvgypc-Ed25519"
val message_to_enc = "hello😀!"
val message_to_dec = "MsgBegin_8qD2TrjJMvhQPydfWVgbLkBb1GgaFe69fS6iUtBjxWAA9uvU3xLYZU6SFbi2VHjQL9dpxTuaYer6zoM6RkWfEY5sTwoRQdjhw5FNeVr1MfgkFRRY8zPwznWQKCT3JaMMdGz5p4QYPmh5bSf961RvS861FR3cd8LKnYSUUAY57T5425bTPwFVq8P3XsTuPFzroxYZ36XjEoSoxm8wKXjih7PBHK3oKtrsfLqvi4XPVwLMz17Virgxb7fthv6oQn5YZZ77ygrpFuSduQT53BcmZSDKEnLQLtZgsCCZjW5DgbQpwjbBGJ8o7yvV4fUFESucfHULDfprg1xm8FmHokDUTy7AzgNASPxpZD2qkCwKFKasgdJBTcu2pxd1FkX5YnWRRcBDi9mVGB4PcuFqn3geov4tt9bVzmpXJ2hEoUHGWqVztmEAdTBxUcDVZEjmqWJbyrHRL4mRCkUreLAs9kJGk6gX4Cd8jTTPG9sdbPy5wAAVS18LpQt7qpqT9iKpdD4xDf6Vmrszmd_EndMsg"
val message_with_sign = "MsgBegin_ciXpWbV9LWtTiBCak4CATF92sMSx6QZSkFF3LhvCDYoKxthfs6kvFj9DFY7WvESsWL6ywC5JHowGr31xx3dNm47vRpBHxDNjyj83TeJDVLY6rtgB7GhEeYCaJe87ozDcPcGopZbtH6DLYB5qjCgHvMCi4ESTteX2ndDYqatuWAjBWyTMRnL3kw8T2tK9ncXr8pREKXWGEtVDyVRa6t89xopacM1Y4tffRC8wuk4ov4d8TD5Yt8EaH5Ytk5DU9d3Mg3CkaerHVQsG4H2rkPVUhXT4nZxWbUXy7TJ3tMhXC6eRRv5pfXHS1eRZAQdcR3HTuxZWsX5tn1qcM1zXj9pV9UgwgFt95VkqkuBXsCQx6hxkfJJUnJ4HXZw5sqosoXRSQknZfYpo87Q7msi7JFuKaRFLVHrtMstah1rRGcVDzVGeLiBEDoPqrwTExAbTpwossrNURCFbBBGujr494A53kKpxHSS1JVZ567Xg7XijSUw7oP57QPyKuaq8GyXmDT2QGv88s5m7EUmacUEZNJtFRyDA2r7wWE5Nig9ikuf1A1iDXnki2HguipjeQo7KtBiGSZaYxbZxyGwnQDioXzih3VraRu5vwCAJ1HFG9kk5DxsvyXbCkaAw3uH_EndMsg"


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