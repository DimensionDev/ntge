using System;
using System.Runtime.InteropServices;
using System.Text;

namespace NtgeCore.Net
{
    public class StringHandle : SafeHandle
    {
        public StringHandle() : base(IntPtr.Zero, true) { }

        public override bool IsInvalid
        {
            get { return false; }
        }

        public string AsString()
        {
            int len = 0;
            while (Marshal.ReadByte(handle,len) != 0) { ++len; }
            byte[] buffer = new byte[len];
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