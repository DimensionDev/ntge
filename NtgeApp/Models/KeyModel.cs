using System.ComponentModel;
using System.IO;
using System.Runtime.CompilerServices;
using System.Threading.Tasks;
using PropertyChanged;

namespace NtgeApp.Models
{
    public class KeyModel : INotifyPropertyChanged
    {
        public KeyModel(string path)
        {
            Path = path;
        }

        public string Path { get; set; }

        [DependsOn(nameof(Path))] public string Name => System.IO.Path.GetFileName(Path);

        // public string Content { get; set; }

        public string? PublicKeyContent { get; set; }
        public string? PrivateKeyContent { get; set; }

        public event PropertyChangedEventHandler? PropertyChanged;

        public async Task EnsurePublicKeyContent()
        {
            if (string.IsNullOrEmpty(PublicKeyContent))
            {
                var result = await File.ReadAllTextAsync(Path + ".pub");
                PublicKeyContent = result.Trim();
            }
        }

        public async Task EnsurePrivateKeyContent()
        {
            if (string.IsNullOrEmpty(PrivateKeyContent))
            {
                var result = await File.ReadAllTextAsync(Path);
                PublicKeyContent = result.Trim();
            }
        }

        protected void OnPropertyChanged([CallerMemberName] string? propertyName = null)
        {
            PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(propertyName));
        }
    }
}