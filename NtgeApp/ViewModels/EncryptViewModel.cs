using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Collections.Specialized;
using System.Linq;
using NtgeApp.Models;
using NtgeCore.Net;
using NtgeCore.Net.Ed25519;
using NtgeCore.Net.Message;
using NtgeCore.Net.X25519;
using PropertyChanged;

namespace NtgeApp.ViewModels
{
    public class EncryptViewModel : ViewModelBase
    {
        public EncryptViewModel()
        {
            Keys.CollectionChanged += KeysOnCollectionChanged;
        }

        public ObservableCollection<EncryptKeyModel> Keys { get; } = new ObservableCollection<EncryptKeyModel>();

        public string? Input { get; set; }

        [DependsOn(nameof(Input))]
        public string Output
        {
            get
            {
                if (string.IsNullOrEmpty(Input))
                {
                    return string.Empty;
                }

                using var encryptor = Encryptor.New(Keys.Select(it => it.X25519PublicKey).ToArray());
                using var message = encryptor.EncryptPlaintext(Input);
                return message.Serialize();
            }
        }

        private void KeysOnCollectionChanged(object sender, NotifyCollectionChangedEventArgs e)
        {
            OnPropertyChanged(Output);
        }

        public void AddKey(string publicKey)
        {
            if (Keys.Any(it => it.Content == publicKey))
            {
                return;
            }

            try
            {
                using var ed25519PublicKey = Ed25519PublicKey.Deserialize(publicKey);
                Keys.Add(new EncryptKeyModel(publicKey, ed25519PublicKey.ToX25519()));
            }
            catch (NtgeException e)
            {
            }
        }

        public void RemoveKey(EncryptKeyModel publicKey)
        {
            Keys.Remove(publicKey);
            publicKey?.Dispose();
        }
    }
}