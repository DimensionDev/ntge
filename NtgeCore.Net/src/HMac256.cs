using System.Text;
using NtgeCore.Net.Ed25519;

namespace NtgeCore.Net
{
    public static class HMac256
    {
        public static string Calculate(Ed25519PublicKey publicKey, string data)
        {
            using var result = Native.hmac_utils_hmac256_calculate_using(publicKey.Ptr, Encoding.UTF8.GetBytes(data));
            return result.AsString();
        }
    }
}