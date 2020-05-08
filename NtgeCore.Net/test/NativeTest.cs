using System;
using Xunit;

namespace NtgeCore.Net.Test
{
    public class NativeTest
    {
        [Fact]
        public void Test_Ed25519PrivateKey()
        {
            var ptr = Native.newEd25519PrivateKey();
            Assert.False(IntPtr.Zero == ptr);
            using var str = Native.serializeEd25519PrivateKey(ptr);
            var result = str.AsString();
            Assert.False(string.IsNullOrEmpty(result));
            var desPtr = Native.deserializeEd25519PrivateKey(result);
            Assert.False(IntPtr.Zero == desPtr);
            using var str2 = Native.serializeEd25519PrivateKey(desPtr);
            var result2 = str.AsString();
            Assert.True(result == result2);
        }
    }
}
