using System;
using System.Text;
using NtgeCore.Net.Message;
using NtgeCore.Net.X25519;

namespace NtgeCore.Net.Ed25519
{
    public class Ed25519PublicKey : RustObject
    {
        private String? _keyId;
        internal Ed25519PublicKey(IntPtr ptr) : base(ptr)
        {
        }

        public static Ed25519PublicKey Deserialize(string value)
        {
            var ptr = Native.deserializeEd25519PublicKey(Encoding.UTF8.GetBytes(value));
            if (ptr == IntPtr.Zero)
            {
                throw new NtgeException("Can not deserialize private key");
            }
            return new Ed25519PublicKey(ptr);
        }

        public String KeyId 
        {
            get 
            {
                if (_keyId == null)
                {
                    using var handle = Native.publicKeyKeyId(Ptr);
                    _keyId = handle.AsString();
                }
                return _keyId;
            }
        }

        public string Serialize()
        {
            using var result = Native.serializeEd25519PublicKey(Ptr);
            return result.AsString();
        }

        public X25519PublicKey ToX25519()
        {
            return new X25519PublicKey(Native.ed25519PublicKeyToX25519(Ptr));
        }

        public bool Verify(string message, string signature)
        {
            var result = Native.ed25519_public_key_verify(Ptr, Encoding.UTF8.GetBytes(message), Encoding.UTF8.GetBytes(signature));
            return result == 0;
        }

        public override void Dispose()
        {
            Native.destroyEd25519PublicKey(Ptr);
        }

        public override string ToString()
        {
            return Serialize();
        }
    }
}