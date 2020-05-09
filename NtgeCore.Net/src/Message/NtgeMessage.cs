using System;
using System.Text;
using NtgeCore.Net.Ed25519;

namespace NtgeCore.Net.Message
{
    public class NtgeMessage : RustObject
    {
        internal NtgeMessage(IntPtr ptr) : base(ptr)
        {
        }

        public static NtgeMessage Deserialize(string input)
        {
            var ptr = Native.deserializeMessage(Encoding.UTF8.GetBytes(input));
            if (ptr == IntPtr.Zero)
            {
                throw new NtgeException("Can not deserialize message");
            }
            return new NtgeMessage(ptr);
        }

        public bool VerifySignature(Ed25519PublicKey publicKey)
        {
            return Native.messageDecryptorVerifySignature(Ptr, publicKey.Ptr);
        }

        public string Serialize()
        {
            using var result = Native.serializeMessage(Ptr);
            return result.AsString();
        }

        public override void Dispose()
        {
            Native.destroyMessage(Ptr);
        }

        public override string ToString()
        {
            return Serialize();
        }
    }
}