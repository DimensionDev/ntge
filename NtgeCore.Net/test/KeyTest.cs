using System;
using NtgeCore.Net.Ed25519;
using Xunit;

namespace NtgeCore.Net.Test
{
    public class KeyTest
    {
        const string test_publicKey = "pub1pacz8fthpmemnvmehnk6n6y2vcc367j65w8c04xfz9qulhyzw2vqvmy8sg-Ed25519";
        const string test_privateKey = "pri15umhflv9mjwj7spxdjq4chzjacfgryf8hgak9727k0gl2px84kkssvgypc-Ed25519";

        [Fact]
        public void CreateEd25519Keypair()
        {
            using var keypair = Ed25519Keypair.New();
            Assert.True(keypair.Ptr != IntPtr.Zero);
        }

        [Fact]
        public void CreateEd25519PrivateKey()
        {
            using var privateKey = Ed25519PrivateKey.New();
            Assert.True(privateKey.Ptr != IntPtr.Zero);
        }

        [Fact]
        public void SerializeEd25519PrivateKey()
        {
            using var privateKey = Ed25519PrivateKey.New();
            var result = privateKey.Serialize();
            Assert.False(string.IsNullOrEmpty(result));
        }

        [Fact]
        public void SerializeEd25519Keypair()
        {
            using var keypair = Ed25519Keypair.New();
            using var privateKey = keypair.PrivateKey;
            var privateKeyResult = privateKey.Serialize();
            Assert.False(string.IsNullOrEmpty(privateKeyResult));
            using var publicKey = keypair.PublicKey;
            var publicKeyResult = publicKey.Serialize();
            Assert.False(string.IsNullOrEmpty(publicKeyResult));
        }

        [Fact]
        public void DeserializeEd25519PublicKey()
        {
            using var publicKey = Ed25519PublicKey.Deserialize(test_publicKey);
            Assert.True(publicKey.Ptr != IntPtr.Zero);
            Assert.Throws<NtgeException>(() => Ed25519PublicKey.Deserialize(""));
            Assert.Throws<NtgeException>(() => Ed25519PublicKey.Deserialize("hello!"));
            Assert.Throws<NtgeException>(() => Ed25519PublicKey.Deserialize("😉😊😋😎🤣😄😅😗"));
            Assert.Throws<NtgeException>(() => Ed25519PublicKey.Deserialize(test_privateKey));
            Assert.Throws<NtgeException>(() => Ed25519PublicKey.Deserialize("pri15umhflv9mjwj7spxdjq4chzjacfgryf8hgak9727k0gl2px84kkssvgypc-Ed25518"));
        }

        [Fact]
        public void DeserializeEd25519PrivateKey()
        {
            using var privateKey = Ed25519PrivateKey.Deserialize(test_privateKey);
            Assert.True(privateKey.Ptr != IntPtr.Zero);
            Assert.Throws<NtgeException>(() => Ed25519PrivateKey.Deserialize(""));
            Assert.Throws<NtgeException>(() => Ed25519PrivateKey.Deserialize("hello!"));
            Assert.Throws<NtgeException>(() => Ed25519PrivateKey.Deserialize("😉😊😋😎🤣😄😅😗"));
            Assert.Throws<NtgeException>(() => Ed25519PrivateKey.Deserialize(test_publicKey));
            Assert.Throws<NtgeException>(() => Ed25519PrivateKey.Deserialize("pub1pacz8fthpmemnvmehnk6n6y2vcc367j65w8c04xfz9qulhyzw2vqvmy8sg-Ed25518"));
        }

        [Fact]
        public void CreateEd25519KeypairFromEd25519PrivateKey()
        {
            using var privateKey = Ed25519PrivateKey.New();
            using var keypair = Ed25519Keypair.FromPrivateKey(privateKey);
            Assert.True(keypair.Ptr != IntPtr.Zero);
        }

        [Fact]
        public void ConvertEd25519PublicKeyToX25519PublicKey()
        {
            using var publicKey = Ed25519PublicKey.Deserialize(test_publicKey);
            using var x25519Key = publicKey.ToX25519();
            Assert.True(x25519Key.Ptr != IntPtr.Zero);
        }
        [Fact]
        public void ConvertEd25519PrivateKeyToX25519PrivateKey()
        {
            using var privateKey = Ed25519PrivateKey.Deserialize(test_privateKey);
            using var x25519Key = privateKey.ToX25519();
            Assert.True(x25519Key.Ptr != IntPtr.Zero);
        }

        [Fact]
        public void GetEd25519PublicKeyId()
        {
            using var publicKey = Ed25519PublicKey.Deserialize(test_publicKey);
            Assert.True(!string.IsNullOrEmpty(publicKey.KeyId));
        }
    }
}
