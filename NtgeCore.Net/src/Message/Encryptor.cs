using System;
using System.Text;
using NtgeCore.Net.Ed25519;
using NtgeCore.Net.X25519;

namespace NtgeCore.Net.Message
{
    public class Encryptor : RustObject
    {
        private IntPtr _keysPtr;
        internal Encryptor(IntPtr ptr, IntPtr keysPtr) : base(ptr)
        {
            _keysPtr = keysPtr;
        }

        public static Encryptor New(params X25519PublicKey[] publicKey)
        {
            var keysPtr = Native.newArrayForX25519PublicKey();
            foreach (var item in publicKey)
            {
                Native.pushArrayX25519PublicKey(keysPtr, item.Ptr);
            }
            return new Encryptor(Native.newMessageEncryptor(keysPtr), keysPtr);
        }

        public NtgeMessage EncryptPlaintext(string text, Ed25519PrivateKey? signatureKey = null)
        {
            return new NtgeMessage(Native.encryptPlaintext(Encoding.UTF8.GetBytes(text), Ptr, signatureKey?.Ptr ?? IntPtr.Zero));
        }

        public NtgeMessage EncryptPlaintextWithExtra(string text, string extra, Ed25519PrivateKey? signatureKey = null)
        {
            return new NtgeMessage(Native.encryptPlaintextWithExtra(Ptr, Encoding.UTF8.GetBytes(text), Encoding.UTF8.GetBytes(extra) , signatureKey?.Ptr ?? IntPtr.Zero));
        }

        public override void Dispose()
        {
            Native.destroyMessageEncryptor(Ptr);
            Native.destroyArrayX25519PublicKey(_keysPtr);
        }
    }
}