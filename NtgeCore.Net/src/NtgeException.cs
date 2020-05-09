namespace NtgeCore.Net
{
    [System.Serializable]
    public class NtgeException : System.Exception
    {
        public NtgeException() { }
        public NtgeException(string message) : base(message) { }
        public NtgeException(string message, System.Exception inner) : base(message, inner) { }
        protected NtgeException(
            System.Runtime.Serialization.SerializationInfo info,
            System.Runtime.Serialization.StreamingContext context) : base(info, context) { }
    }
}