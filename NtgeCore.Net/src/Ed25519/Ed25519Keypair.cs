using System;

namespace NtgeCore.Net.Ed25519
{
    public class Ed25519Keypair : RustObject
    {
        private Ed25519PrivateKey? _privateKey;
        private Ed25519PublicKey? _publicKey;

        internal Ed25519Keypair(IntPtr ptr) : base(ptr)
        {
        }

        public Ed25519PrivateKey PrivateKey
        {
            get
            {
                if (_privateKey == null)
                {
                    _privateKey = new Ed25519PrivateKey(Native.getPrivateKeyFromEd25519Keypair(Ptr));
                }
                return _privateKey;
            }
        }

        public Ed25519PublicKey PublicKey
        {
            get
            {
                if (_publicKey == null)
                {
                    _publicKey = new Ed25519PublicKey(Native.getPublicKeyFromEd25519Keypair(Ptr));
                }
                return _publicKey;
            }
        }

        public static Ed25519Keypair New()
        {
            return new Ed25519Keypair(Native.newEd25519Keypair());
        }

        public static Ed25519Keypair FromPrivateKey(Ed25519PrivateKey privateKey)
        {
            return new Ed25519Keypair(Native.getEd25519KeypairFromPrivateKey(privateKey.Ptr));
        }

        public override void Dispose()
        {
            Native.destroyEd25519Keypair(Ptr);
        }
    }
}