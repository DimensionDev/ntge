using System.IO;
using System.Threading.Tasks;
using Avalonia.Controls;
using Avalonia.Input;
using Avalonia.Interactivity;
using Avalonia.Markup.Xaml;
using Avalonia.Threading;
using NtgeApp.Common;
using NtgeCore.Net.Ed25519;
using PropertyChanged;

namespace NtgeApp.Dialogs
{
    [DoNotNotify]
    public class CreateKeyDialog : Dialog
    {
        public CreateKeyDialog()
        {
            InitializeComponent();
            var textBox = this.Find<TextBox>("CreateKeyTextBox");
            textBox.KeyDown += TextBoxOnKeyDown;
            Dispatcher.UIThread.Post(() => textBox.Focus());
        }

        private async void TextBoxOnKeyDown(object? sender, KeyEventArgs e)
        {
            if (!(sender is TextBox textBox))
            {
                return;
            }

            if (e.Key == Key.Return)
            {
                e.Handled = true;
                if (!string.IsNullOrEmpty(textBox.Text))
                {
                    textBox.IsEnabled = false;
                    await CreateKey();
                }
            }
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
            await CreateKey();
        }

        private async Task CreateKey()
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
            await File.WriteAllTextAsync(Path.Combine(Consts.HomeDir, Consts.NtgeFolder, $"{text}.pub"),
                publicKey.Serialize());
            Close();
        }
    }
}