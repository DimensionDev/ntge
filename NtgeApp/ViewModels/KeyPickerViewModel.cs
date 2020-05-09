using NtgeApp.Models;

namespace NtgeApp.ViewModels
{
    public class KeyPickerViewModel : KeysViewModel
    {
        public KeyModel? SelectedModel { get; set; }
        public string? CustomKeyContent { get; set; }
    }
}