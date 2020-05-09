using System.IO;
using Avalonia.Controls;
using Avalonia.Controls.ApplicationLifetimes;
using Avalonia.Interactivity;
using Avalonia.Markup.Xaml;
using NtgeApp.ViewModels;
using PropertyChanged;

namespace NtgeApp.Dialogs
{
    [DoNotNotify]
    public class KeyPickerDialog : Window
    {
        public KeyPickerDialog()
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
            if (!(DataContext is KeyPickerViewModel viewModel))
            {
                return;
            }
            var tabControl = this.Find<TabControl>("PickerTabControl");
            if (tabControl.SelectedIndex == 0)
            {
                if (viewModel.SelectedModel != null)
                {
                    if (viewModel.SelectedModel.Content == null)
                    {
                        viewModel.SelectedModel.Content = await File.ReadAllTextAsync(viewModel.SelectedModel.Path);
                    }
                    this.Close(viewModel.SelectedModel.Content);
                }
            }
            else
            {
                this.Close(viewModel.CustomKeyContent);
            }
        }
    }
}