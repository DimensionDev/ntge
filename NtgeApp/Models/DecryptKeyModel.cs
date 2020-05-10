using System;
using NtgeCore.Net.X25519;

namespace NtgeApp.Models
{
    public class DecryptKeyModel : IDisposable
    {
        public DecryptKeyModel(string content, X25519PrivateKey x25519PrivateKey)
        {
            Content = content;
            X25519PrivateKey = x25519PrivateKey;
        }

        public string Content { get; }
        public X25519PrivateKey X25519PrivateKey { get; }

        public void Dispose()
        {
            X25519PrivateKey.Dispose();
        }
    }
}