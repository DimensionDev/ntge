using Xunit;

namespace NtgeCore.Net.Test
{
    public class HMac256Test
    {
        [Fact]
        public void TestName()
        {
            using var publicKey = Ed25519.Ed25519PublicKey.Deserialize("pub1ryd8qreac4s2tz0ect98sn5hpjc7254qu6ea748urn3u2mxygmfqtx0hvq-Ed25519");
            var result = HMac256.Calculate(publicKey, "Hello, World!");
            Assert.Equal(result, "45cf8a356f3cdebda7ccc08fdea82a7112f9ec14bae66f2e715e48ccd5ec2541");
        }
    }
}