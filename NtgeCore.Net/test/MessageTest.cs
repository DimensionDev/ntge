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
        const string message_to_dec = "MsgBegin_ciXpWbV9LWtTiBCak4CATF92sMSx6QZSkFF3LhvCDYoKxthfs6kvFj9DFY7WvESsWL6ywC5JHowGr31xx3dNm47vRpBHxDNjyj83TeJDVLY6rtgAgphpYx6rkgNM7Rvg73vq8scfALTkKuUkkwDP8pKF8RTHSV4Uidfsa22GBrJ6medq5V2UPYcF3gEZY7mySEYdaLiiTpqQhKcFH8etNG3f4tGdCCDUNMu63jehSvJrHJRczuxB676hC4sGwnLD5u4KZ8UhP9hRsHpL45hzgTHT1NRSe3DqZ5FEXonnU6Qc4gtYS7L6y5doyrFsq6wdRCSRqFCLGQnPsVw8efzowk4zSJRdUQEuzEgK4zwFN7XuwHZ7hXnrRcg5ma5Gsszky13wPQ858QXHVVN6999a376rpoMseocPzTBrX8fufBWejscXjRgQZuTkyYfbZ9SLD34MumAkF7ZNYan7WA6wk85HXQnxTbDb4j8XGs6nsEKTciqXCfKt7LBnmt8XhmKXQDLxC48e9G8HjfJ9WhqU6xUSzD2pFfGoSW1S4sX4AWtMNBpzNCmh8wgprHpRru6G4jA3qDdJ44swzhZW23fzPZCfo4E4zjYu9PXP1eAa1PYo61R33wCshjV_EndMsg";


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