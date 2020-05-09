using System;

namespace NtgeCore.Net.X25519
{
    public class X25519PrivateKey : RustObject
    {
        internal X25519PrivateKey(IntPtr ptr) : base(ptr)
        {
        }

        public override void Dispose()
        {
            Native.destroyX25519PrivateKey(Ptr);
        }
    }

}