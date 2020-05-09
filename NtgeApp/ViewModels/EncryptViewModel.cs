using System.Collections.ObjectModel;
using System.Linq;
using NtgeCore.Net.Ed25519;
using NtgeCore.Net.Message;
using PropertyChanged;

namespace NtgeApp.ViewModels
{
    public class EncryptViewModel : ViewModelBase
    {
        public ObservableCollection<string> Keys { get; } = new ObservableCollection<string>();

        public string Input { get; set; }

        [DependsOn(nameof(Input))]
        public string Output
        {
            get
            {
                return Encryptor.New(Keys.Select(it => Ed25519PublicKey.Deserialize(it).ToX25519()).ToArray())
                    .EncryptPlaintext(Input).Serialize();
            }
        }
    }
}