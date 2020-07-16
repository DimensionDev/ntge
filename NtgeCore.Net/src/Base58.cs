using System.Text;

namespace NtgeCore.Net
{
    public static class Base58
    {
        public static string Encode(string value)
        {
            using var result = Native.base58_encode(Encoding.UTF8.GetBytes(value));
            return result.AsString();
        }

        public static string Decode(string value)
        {
            using var result = Native.base58_decode(Encoding.UTF8.GetBytes(value));
            return result.AsString();
        }
    }
}