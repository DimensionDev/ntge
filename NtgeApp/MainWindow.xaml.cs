using Avalonia.Controls;
using Avalonia.Markup.Xaml;
using PropertyChanged;

namespace NtgeApp
{
    [DoNotNotify]
    public class MainWindow : Window
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