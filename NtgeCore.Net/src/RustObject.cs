using System;
using System.Runtime.CompilerServices;

[assembly:InternalsVisibleTo("NtgeCore.Net.Test")]
namespace NtgeCore.Net
{
    public abstract class RustObject : IDisposable
    {
        internal IntPtr Ptr { get; }

        protected RustObject(IntPtr ptr)
        {
            Ptr = ptr;
        }

        public abstract void Dispose();
    }
}