using System.IO;
using Avalonia.Controls;
using Avalonia.Interactivity;
using Avalonia.Markup.Xaml;
using NtgeApp.Common;
using NtgeCore.Net.Ed25519;
using PropertyChanged;

namespace NtgeApp.Dialogs
{
    [DoNotNotify]
    public class CreateKeyDialog : Window
    {
        public CreateKeyDialog()
        {
            InitializeComponent();
        }

        private void InitializeComponent()
        {
            AvaloniaXamlLoader.Load(this);
        }
        
        private void OnCancelClicked(object? sender, RoutedEventArgs e)
        {
            Close();
        }
        
        private async void OnOkClicked(object? sender, RoutedEventArgs e)
        {
            var textBlock = this.Find<TextBox>("CreateKeyTextBox");
            var text = textBlock.Text;
            if (string.IsNullOrEmpty(text))
            {
                return;
            }

            using var keypair = Ed25519Keypair.New();
            using var publicKey = keypair.PublicKey;
            using var privateKey = keypair.PrivateKey;
            await File.WriteAllTextAsync(Path.Combine(Consts.HomeDir, Consts.NtgeFolder, text), privateKey.Serialize());
            await File.WriteAllTextAsync(Path.Combine(Consts.HomeDir, Consts.NtgeFolder, $"{text}.pub"), publicKey.Serialize());
            this.Close();
        }
    }
}