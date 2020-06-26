using System;
using NtgeCore.Net.Ed25519;
using NtgeCore.Net.Message;
using Xunit;

namespace NtgeCore.Net.Test
{
    public class MessageTest
    {
        const string test_publicKey = "pub1pacz8fthpmemnvmehnk6n6y2vcc367j65w8c04xfz9qulhyzw2vqvmy8sg-Ed25519";
        const string test_privateKey = "pri15umhflv9mjwj7spxdjq4chzjacfgryf8hgak9727k0gl2px84kkssvgypc-Ed25519";
        const string hello_publicKey = "pub1e35qxm0zr87tf05lss2sajt0wp3p8q4nak3lzw47aa80wglwm36q6zhth3-Ed25519";
        const string hello_privateKey = "pri156qr745at67g28wsq50n6r5qzkcegunwqkkg53wnuprc8w5cpgnsvgl6lp-Ed25519";
        const string message_to_enc = "hello😀!";
        const string extra_message_to_enc = "world😀!";
        const string message_to_dec = "MsgBeginIIIIIRrJXG8oAk4L9EYvjm3pEmTWTB9jaqLFRaHUqDY6kz6xad2oqrSV7o6TVqkrP1kHiqCNHVFaiPVLjBCAwPTagSZ69FcEskq6mnA4ZzDDb1DZeBa6Z3WVAmJaqj8ZRSd8YSB5k9QGXZrUMJBX94iQ86KEAoWF6URFy8Vryr4wV82kjYnSDQKJShthYCxab7PWNwt5cs7bWpoqsFisn33n4NdCeUAZZ4MGdnqFGWtgBfcQKHWNHRj4UvZvdGYpaRatGDtmGoKX133MRmV3FmIIIIIEndMsg";


        [Fact]
        public void DeserializeMessage()
        {
            using var message = NtgeMessage.Deserialize(message_to_dec);
            Assert.True(message.Ptr != IntPtr.Zero);
            Assert.Throws<NtgeException>(() => NtgeMessage.Deserialize(""));
            Assert.Throws<NtgeException>(() => NtgeMessage.Deserialize("hello!"));
            Assert.Throws<NtgeException>(() => NtgeMessage.Deserialize("😉😊😋😎🤣😄😅😗"));
            Assert.Throws<NtgeException>(() => NtgeMessage.Deserialize(test_publicKey));
        }

        [Fact]
        public void SerializeMessage()
        {
            using var message = NtgeMessage.Deserialize(message_to_dec);
            var result = message.Serialize();
            Assert.False(string.IsNullOrEmpty(result));
        }

        [Fact]
        public void EncryptMessage()
        {
            using var ed25519PublicKey = Ed25519PublicKey.Deserialize(test_publicKey);
            using var x25519PublicKey = ed25519PublicKey.ToX25519();
            using var encryptor = Encryptor.New(x25519PublicKey);
            using var message = encryptor.EncryptPlaintext(message_to_enc);
            Assert.True(message.Ptr != IntPtr.Zero);
        }

        [Fact]
        public void TestTimestamp()
        {
            using var ed25519PublicKey = Ed25519PublicKey.Deserialize(test_publicKey);
            using var x25519PublicKey = ed25519PublicKey.ToX25519();
            using var encryptor = Encryptor.New(x25519PublicKey);
            using var message = encryptor.EncryptPlaintext(message_to_enc);
            Assert.True(message.Ptr != IntPtr.Zero);
            Assert.True(message.Timestamp != default);
        }

        [Fact]
        public void ShouldGetMessageFileKey()
        {
            using var message = NtgeMessage.Deserialize(message_to_dec);
            using var decryptor = Decryptor.New(message);
            using var ed25519PrivateKey = Ed25519PrivateKey.Deserialize(hello_privateKey);
            using var x25519PrivateKey = ed25519PrivateKey.ToX25519();
            using var fileKey = decryptor.GetFileKey(x25519PrivateKey);
            Assert.True(fileKey.Ptr != IntPtr.Zero);
        }

        [Fact]
        public void ShouldVerifyMessageMac()
        {
            using var message = NtgeMessage.Deserialize(message_to_dec);
            using var decryptor = Decryptor.New(message);
            using var ed25519PrivateKey = Ed25519PrivateKey.Deserialize(hello_privateKey);
            using var x25519PrivateKey = ed25519PrivateKey.ToX25519();
            using var fileKey = decryptor.GetFileKey(x25519PrivateKey);
            Assert.True(decryptor.VerifyMessageMac(fileKey));
        }

        [Fact]
        public void DecryptMessage()
        {
            using var message = NtgeMessage.Deserialize(message_to_dec);
            using var decryptor = Decryptor.New(message);
            using var ed25519PrivateKey = Ed25519PrivateKey.Deserialize(hello_privateKey);
            using var x25519PrivateKey = ed25519PrivateKey.ToX25519();
            using var fileKey = decryptor.GetFileKey(x25519PrivateKey);
            var result = decryptor.DecryptPayload(fileKey);
            Assert.Equal(message_to_enc, result);
        }

        [Fact]
        public void EncryptAndDecrypt()
        {
            using var ed25519PrivateKey = Ed25519PrivateKey.New();
            using var x25519PrivateKey = ed25519PrivateKey.ToX25519();
            using var ed25519PublicKey = ed25519PrivateKey.PublicKey;
            using var x25519PublicKey = ed25519PublicKey.ToX25519();
            using var encryptor = Encryptor.New(x25519PublicKey);
            using var message = encryptor.EncryptPlaintext(message_to_enc);
            Assert.True(message.Ptr != IntPtr.Zero);
            var msgStr = message.Serialize();
            Assert.False(string.IsNullOrEmpty(msgStr));
            using var decMessage = NtgeMessage.Deserialize(msgStr);
            using var decryptor = Decryptor.New(decMessage);
            using var fileKey = decryptor.GetFileKey(x25519PrivateKey);
            var result = decryptor.DecryptPayload(fileKey);
            Assert.Equal(message_to_enc, result);
        }

        [Fact]
        public void EncryptMessageWithSignature()
        {
            using var ed25519PublicKey = Ed25519PublicKey.Deserialize(test_publicKey);
            using var x25519PublicKey = ed25519PublicKey.ToX25519();
            using var ed25519PrivateKey = Ed25519PrivateKey.Deserialize(test_privateKey);
            using var encryptor = Encryptor.New(x25519PublicKey);
            using var message = encryptor.EncryptPlaintext(message_to_enc, ed25519PrivateKey);
            Assert.True(message.Ptr != IntPtr.Zero);
        }

        [Fact]
        public void EncryptAndDecryptWithExtra()
        {
            using var ed25519PrivateKey = Ed25519PrivateKey.New();
            using var x25519PrivateKey = ed25519PrivateKey.ToX25519();
            using var ed25519PublicKey = ed25519PrivateKey.PublicKey;
            using var x25519PublicKey = ed25519PublicKey.ToX25519();
            using var encryptor = Encryptor.New(x25519PublicKey);
            using var message = encryptor.EncryptPlaintextWithExtra(message_to_enc, extra_message_to_enc);
            Assert.True(message.Ptr != IntPtr.Zero);
            var msgStr = message.Serialize();
            Assert.False(string.IsNullOrEmpty(msgStr));
            using var decMessage = NtgeMessage.Deserialize(msgStr);
            using var decryptor = Decryptor.New(decMessage);
            using var fileKey = decryptor.GetFileKey(x25519PrivateKey);
            var result = decryptor.DecryptPayload(fileKey);
            var extraResult = decryptor.DecryptPayloadExtra(fileKey);
            Assert.Equal(message_to_enc, result);
            Assert.Equal(extra_message_to_enc, extraResult);
        }
    }
}