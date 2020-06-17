using System;
using NtgeCore.Net.X25519;

namespace NtgeCore.Net.Message
{
    public class Decryptor : RustObject
    {
        internal Decryptor(IntPtr ptr) : base(ptr)
        {
        }

        public static Decryptor New(NtgeMessage message)
        {
            return new Decryptor(Native.newMessageDecryptor(message.Ptr));
        }

        public bool VerifyMessageMac(X25519FileKey fileKey)
        {
            return Native.messageDecryptorVerifyMessageMac(Ptr, fileKey.Ptr);
        }

        public X25519FileKey GetFileKey(X25519PrivateKey privateKey)
        {
            return new X25519FileKey(Native.messageDecryptorDecryptFileKey(Ptr, privateKey.Ptr));
        }

        public string DecryptPayload(X25519FileKey fileKey)
        {
            using var result = Native.messageDecryptorDecryptPayload(Ptr, fileKey.Ptr);
            return result.AsString();
        }

        public string DecryptPayloadExtra(X25519FileKey fileKey)
        {
            using var result = Native.decryptMessageExtra(Ptr, fileKey.Ptr);
            return result.AsString();
        }

        public override void Dispose()
        {
            Native.destroyMessageDecryptor(Ptr);
        }
    }
}