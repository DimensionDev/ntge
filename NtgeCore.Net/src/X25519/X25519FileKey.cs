using System;

namespace NtgeCore.Net.X25519
{
    public class X25519FileKey : RustObject
    {
        internal X25519FileKey(IntPtr ptr) : base(ptr)
        {
        }

        public override void Dispose()
        {
            Native.destroyX25519FileKey(Ptr);
        }
    }

}