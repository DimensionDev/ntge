namespace NtgeApp.ViewModels
{
    class MainViewModel : ViewModelBase
    {
        public string[] Menus { get; } = new[]
        {
            "Keys",
            "Encrypt",
            "Decrypt",
            "Sign",
            "Verify"
        };
    }
}