using Avalonia;
using Avalonia.Controls;
using Avalonia.Controls.ApplicationLifetimes;
using Avalonia.Interactivity;
using Avalonia.Markup.Xaml;
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

            if (sender is Button button && button.DataContext is string value)
            {
                viewModel.Keys.Remove(value);
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
                var result = await dialog.ShowDialog<KeyModel>(desktop.MainWindow);
                if (result != null)
                {
                    await result.EnsurePublicKeyContent();
                    if (!string.IsNullOrEmpty(result.PublicKeyContent) &&
                        !viewModel.Keys.Contains(result.PublicKeyContent))
                    {
                        viewModel.Keys.Add(result.PublicKeyContent);
                    }
                }
            }
        }

        private void InitializeComponent()
        {
            AvaloniaXamlLoader.Load(this);
        }
    }
}