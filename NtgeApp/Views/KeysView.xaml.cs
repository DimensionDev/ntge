using Avalonia.Controls;
using Avalonia.Controls.ApplicationLifetimes;
using Avalonia.Interactivity;
using Avalonia.Markup.Xaml;
using NtgeApp.Dialogs;
using PropertyChanged;

namespace NtgeApp.Views
{
    [DoNotNotify]
    public class KeysView : UserControl
    {
        public KeysView()
        {
            InitializeComponent();
        }

        private async void OnCreateClicked(object? sender, RoutedEventArgs e)
        {
            if (App.Current.ApplicationLifetime is IClassicDesktopStyleApplicationLifetime desktop)
            {
                var dialog = new CreateKeyDialog();
                await dialog.ShowDialog(desktop.MainWindow);
            }
        }
        
        private void InitializeComponent()
        {
            AvaloniaXamlLoader.Load(this);
        }
    }
}