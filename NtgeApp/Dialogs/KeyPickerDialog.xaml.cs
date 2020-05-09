using Avalonia.Controls;
using Avalonia.Interactivity;
using Avalonia.Markup.Xaml;
using NtgeApp.Models;
using NtgeApp.ViewModels;
using PropertyChanged;

namespace NtgeApp.Dialogs
{
    [DoNotNotify]
    public class KeyPickerDialog : Dialog
    {
        public KeyPickerDialog()
        {
            InitializeComponent();
        }

        private void OnItemDoubleTapped(object? sender, RoutedEventArgs e)
        {
            if (sender is Grid view && view.DataContext is KeyModel model)
            {
                e.Handled = true;
                Close(model);
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

        private void OnOkClicked(object? sender, RoutedEventArgs e)
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
                    Close(viewModel.SelectedModel);
                }
            }
            else
            {
                Close(viewModel.CustomKeyContent);
            }
        }
    }
}