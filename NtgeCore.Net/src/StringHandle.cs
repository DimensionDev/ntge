using System;
using System.Runtime.InteropServices;
using System.Text;

namespace NtgeCore.Net
{
    internal class StringHandle : SafeHandle
    {
        public StringHandle(IntPtr ptr) : this()
        {
            SetHandle(ptr);
        }

        public StringHandle() : base(IntPtr.Zero, true)
        {

        }

        public override bool IsInvalid => false;

        public string AsString()
        {
            var len = 0;
            while (Marshal.ReadByte(handle, len) != 0)
            {
                ++len;
            }
            var buffer = new byte[len];
            Marshal.Copy(handle, buffer, 0, buffer.Length);
            return Encoding.UTF8.GetString(buffer);
        }

        protected override bool ReleaseHandle()
        {

            Native.free_string(handle);
            return true;
        }
    }
}