using Avalonia.Controls;
using Avalonia.Input;
using PropertyChanged;

namespace NtgeApp.Dialogs
{
    [DoNotNotify]
    public class Dialog : Window
    {
        public Dialog()
        {
            SystemDecorations = SystemDecorations.BorderOnly;
            WindowStartupLocation = WindowStartupLocation.CenterOwner;
            ShowInTaskbar = false;
        }

        protected override void OnKeyDown(KeyEventArgs e)
        {
            base.OnKeyDown(e);
            if (e.Key == Key.Escape)
            {
                e.Handled = true;
                Close();
            }
        }
    }
}