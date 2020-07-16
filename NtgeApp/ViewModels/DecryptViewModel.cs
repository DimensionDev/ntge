using System;
using System.Collections.ObjectModel;
using System.Collections.Specialized;
using NtgeApp.Models;
using NtgeCore.Net;
using NtgeCore.Net.Ed25519;
using NtgeCore.Net.Message;
using PropertyChanged;

namespace NtgeApp.ViewModels
{
    public class DecryptViewModel : ViewModelBase
    {
        public DecryptKeyModel? DecryptKeyModel { get; private set; }
        
        public string? Input { get; set; }
        
        public string? Output { get; set; }
        
        public void OnInputChanged()
        {
            Decrypt();
        }

        public void OnDecryptKeyModelChanged()
        {
            Decrypt();
        }

        void Decrypt()
        {
            if (string.IsNullOrEmpty(Input) || DecryptKeyModel == null)
            {
                Output = string.Empty;
                return;
            }
            try
            {
                using var message = NtgeMessage.Deserialize(Input);
                using var decryptor = Decryptor.New(message);
                using var fileKey = decryptor.GetFileKey(DecryptKeyModel.X25519PrivateKey);
                Output = decryptor.DecryptPayload(fileKey);
            }
            catch (NtgeException e)
            {
                Output = e.Message;
            }
        }

        public void SetKey(string privateKeyContent)
        {
            DecryptKeyModel?.Dispose();
            try
            {
                using var privateKey = Ed25519PrivateKey.Deserialize(privateKeyContent);
                DecryptKeyModel = new DecryptKeyModel(privateKeyContent, privateKey.ToX25519());
            }
            catch (NtgeException e)
            {
                DecryptKeyModel = null;
            }
            
        }
    }
}