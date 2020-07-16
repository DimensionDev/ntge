using System;
using NtgeCore.Net.X25519;

namespace NtgeApp.Models
{
    public class EncryptKeyModel : IDisposable
    {
        public EncryptKeyModel(string content, X25519PublicKey x25519PublicKey)
        {
            Content = content;
            X25519PublicKey = x25519PublicKey;
        }

        public string Content { get; }
        public X25519PublicKey X25519PublicKey { get; }

        public void Dispose()
        {
            X25519PublicKey.Dispose();
        }
    }
}