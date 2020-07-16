using Xunit;

namespace NtgeCore.Net.Test
{
    public class Base58Test
    {
        [Fact]
        public void Encode()
        {
            Assert.Equal("D7LMXYjYZ7cDaGe8bS", Base58.Encode("Hello, World!"));
        }

        [Fact]
        public void Decode()
        {
            Assert.Equal("Hello, World!", Base58.Decode("D7LMXYjYZ7cDaGe8bS"));
        }
    }
}