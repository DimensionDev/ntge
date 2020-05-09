using System.Collections.ObjectModel;
using System.Collections.Specialized;
using System.Linq;
using NtgeCore.Net.Ed25519;
using NtgeCore.Net.Message;
using PropertyChanged;

namespace NtgeApp.ViewModels
{
    public class EncryptViewModel : ViewModelBase
    {
        public EncryptViewModel()
        {
            Keys.CollectionChanged += KeysOnCollectionChanged;
        }

        public ObservableCollection<string> Keys { get; } = new ObservableCollection<string>();

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

                return Encryptor.New(Keys.Select(it => Ed25519PublicKey.Deserialize(it).ToX25519()).ToArray())
                    .EncryptPlaintext(Input).Serialize();
            }
        }

        private void KeysOnCollectionChanged(object sender, NotifyCollectionChangedEventArgs e)
        {
            OnPropertyChanged(Output);
        }
    }
}