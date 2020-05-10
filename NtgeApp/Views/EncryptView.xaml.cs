using Avalonia;
using Avalonia.Controls;
using Avalonia.Controls.ApplicationLifetimes;
using Avalonia.Interactivity;
using Avalonia.Markup.Xaml;
using Avalonia.Media;
using NtgeApp.Dialogs;
using NtgeApp.Models;
using NtgeApp.ViewModels;
using PropertyChanged;

namespace NtgeApp.Views
{
    [DoNotNotify]
    public class EncryptView : UserControl
    {
        public EncryptView()
        {
            InitializeComponent();
        }

        private void OnRemoveClick(object? sender, RoutedEventArgs e)
        {
            if (!(DataContext is EncryptViewModel viewModel))
            {
                return;
            }

            if (sender is Button button && button.DataContext is EncryptKeyModel value)
            {
                viewModel.RemoveKey(value);
            }
        }

        private async void OnAddClick(object? sender, RoutedEventArgs e)
        {
            if (!(DataContext is EncryptViewModel viewModel))
            {
                return;
            }

            var dialog = new KeyPickerDialog();
            if (Application.Current.ApplicationLifetime is IClassicDesktopStyleApplicationLifetime desktop)
            {
                var result = await dialog.ShowDialog<object>(desktop.MainWindow);
                switch (result)
                {
                    case KeyModel keyModel:
                        await keyModel.EnsurePublicKeyContent();
                        if (!string.IsNullOrEmpty(keyModel.PublicKeyContent))
                        {
                            viewModel.AddKey(keyModel.PublicKeyContent);
                        }
                        break;
                    case string key: 
                        viewModel.AddKey(key);
                        break;
                }
            }
        }

        private void InitializeComponent()
        {
            AvaloniaXamlLoader.Load(this);
        }
    }
}