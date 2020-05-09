using Avalonia;
using Avalonia.Controls;
using Avalonia.Controls.ApplicationLifetimes;
using Avalonia.Interactivity;
using Avalonia.Markup.Xaml;
using NtgeApp.Dialogs;
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

        private async void OnAddClick(object? sender, RoutedEventArgs e)
        {
            if (!(DataContext is EncryptViewModel viewModel))
            {
                return;
            }
            var dialog = new KeyPickerDialog();
            if (App.Current.ApplicationLifetime is IClassicDesktopStyleApplicationLifetime desktop)
            {
                var result = await dialog.ShowDialog<string>(desktop.MainWindow);
                if (!string.IsNullOrEmpty(result))
                {
                    result = result.Trim();
                    if (!viewModel.Keys.Contains(result))
                    {
                        viewModel.Keys.Add(result);
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