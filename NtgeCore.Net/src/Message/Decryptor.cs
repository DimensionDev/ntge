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
            var ptr = Native.messageDecryptorDecryptFileKey(Ptr, privateKey.Ptr);
            if (ptr == IntPtr.Zero)
            {
                throw new NtgeException("Can not get file key");
            }
            return new X25519FileKey(ptr);
        }

        public string DecryptPayload(X25519FileKey fileKey)
        {
            var ptr = Native.messageDecryptorDecryptPayload(Ptr, fileKey.Ptr);
            if (ptr == IntPtr.Zero)
            {
                throw new NtgeException("Can not decrypt payload");
            }
            using var stringHandle = new StringHandle(ptr);
            return stringHandle.AsString();
        }

        public override void Dispose()
        {
            Native.destroyMessageDecryptor(Ptr);
        }
    }
}