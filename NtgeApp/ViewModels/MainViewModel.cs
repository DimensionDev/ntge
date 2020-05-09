using Avalonia;
using Avalonia.Controls;
using Avalonia.Media;
using NtgeApp.Models;
using NtgeApp.Views;

namespace NtgeApp.ViewModels
{
    internal class MainViewModel : ViewModelBase
    {
        public TabItemModel[] Tabs { get; } =
        {
            new TabItemModel("Keys", Application.Current.FindResource("BoxIcons.RegularKey") as GeometryDrawing,
                new KeysView()),
            new TabItemModel("Encrypt",
                Application.Current.FindResource("MaterialDesign.EnhancedEncryption") as GeometryDrawing,
                new EncryptView())
        };
    }
}