using Avalonia.Controls;
using Avalonia.Markup.Xaml;
using PropertyChanged;

namespace NtgeApp
{
    [DoNotNotify]
    public class MainWindow : FluentWindow
    {
        public MainWindow()
        {
            InitializeComponent();
        }

        private void InitializeComponent()
        {
            AvaloniaXamlLoader.Load(this);
        }
    }
}