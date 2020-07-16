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
    public class DecryptView : UserControl
    {
        public DecryptView()
        {
            InitializeComponent();
        }

        private void InitializeComponent()
        {
            AvaloniaXamlLoader.Load(this);
        }
        
        

        private async void OnSelectKeyClick(object? sender, RoutedEventArgs e)
        {
            if (!(DataContext is DecryptViewModel viewModel))
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
                        await keyModel.EnsurePrivateKeyContent();
                        if (!string.IsNullOrEmpty(keyModel.PrivateKeyContent))
                        {
                            viewModel.SetKey(keyModel.PrivateKeyContent);
                        }
                        break;
                    case string key: 
                        viewModel.SetKey(key);
                        break;
                }
            }
        }

    }
}