using System;

namespace NtgeCore.Net.X25519
{
    public class X25519PublicKey : RustObject
    {
        internal X25519PublicKey(IntPtr ptr) : base(ptr)
        {
        }

        public override void Dispose()
        {
            Native.destroyX25519PublicKey(Ptr);
        }
    }

}