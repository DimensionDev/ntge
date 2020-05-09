using System.ComponentModel;
using System.Runtime.CompilerServices;
using JetBrains.Annotations;
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
        
        [DependsOn(nameof(Path))]
        public string Name => System.IO.Path.GetFileName(Path);

        public string Content { get; set; }
        
        public event PropertyChangedEventHandler PropertyChanged;

        protected void OnPropertyChanged([CallerMemberName] string propertyName = null)
        {
            PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(propertyName));
        }
    }
}