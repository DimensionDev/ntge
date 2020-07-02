using System;
using System.Text;
using NtgeCore.Net.Message;
using NtgeCore.Net.X25519;

namespace NtgeCore.Net.Ed25519
{
    public class Ed25519PrivateKey : RustObject
    {
        private Ed25519PublicKey? _publicKey;
        internal Ed25519PrivateKey(IntPtr ptr) : base(ptr)
        {
        }

        public Ed25519PublicKey PublicKey
        {
            get
            {
                if (_publicKey == null)
                {
                    _publicKey = new Ed25519PublicKey(Native.getPublicKeyFromEd25519PrivateKey(Ptr));
                }
                return _publicKey;
            }
        }

        public static Ed25519PrivateKey New()
        {
            return new Ed25519PrivateKey(Native.newEd25519PrivateKey());
        }

        public static Ed25519PrivateKey Deserialize(string value)
        {
            var ptr = Native.deserializeEd25519PrivateKey(Encoding.UTF8.GetBytes(value));
            if (ptr == IntPtr.Zero)
            {
                throw new NtgeException("Can not deserialize private key");
            }
            return new Ed25519PrivateKey(ptr);
        }

        public string Serialize()
        {
            using var result = Native.serializeEd25519PrivateKey(Ptr);
            return result.AsString();
        }

        public X25519PrivateKey ToX25519()
        {
            return new X25519PrivateKey(Native.ed25519PrivateKeyToX25519(Ptr));
        }

        public string Sign(string message)
        {
            using var result = Native.ed25519_private_key_sign(Ptr, Encoding.UTF8.GetBytes(message));
            return result.AsString();
        }

        public override void Dispose()
        {
            Native.destroyEd25519PrivateKey(Ptr);
        }

        public override string ToString()
        {
            return Serialize();
        }
    }
}