using System.Collections.ObjectModel;
using System.IO;
using System.Linq;
using Avalonia.Threading;
using NtgeApp.Common;
using NtgeApp.Models;

namespace NtgeApp.DataSource
{
    public class KeySource
    {
        private readonly FileSystemWatcher _watcher;

        private KeySource()
        {
            LoadKeys();

            _watcher = new FileSystemWatcher();
            _watcher.NotifyFilter = NotifyFilters.FileName | NotifyFilters.CreationTime | NotifyFilters.LastWrite;
            _watcher.Path = Path.Combine(Consts.HomeDir, Consts.NtgeFolder);
            _watcher.Renamed += WatcherOnRenamed;
            _watcher.Created += WatcherOnCreated;
            _watcher.Deleted += WatcherOnDeleted;
            _watcher.EnableRaisingEvents = true;
        }

        public static KeySource Instance { get; } = new KeySource();
        public ObservableCollection<KeyModel> Items { get; } = new ObservableCollection<KeyModel>();

        private void LoadKeys()
        {
            Items.Clear();
            var path = Path.Combine(Consts.HomeDir, Consts.NtgeFolder);
            if (!Directory.Exists(path))
            {
                Directory.CreateDirectory(path);
            }

            var files = Directory.GetFiles(path)
                .Select(it => (path: it, name: Path.GetFileNameWithoutExtension(it)))
                .Where(it => !string.IsNullOrEmpty(it.name))
                .GroupBy(it => it.name)
                .Where(it => it.Count() == 2 && File.Exists(it.FirstOrDefault().path + ".pub"))
                .SelectMany(it => it.Where(tuple => !tuple.path.EndsWith(".pub")))
                .Select(it => it.path);

            foreach (var file in files)
            {
                Items.Add(new KeyModel(file));
            }
        }

        private void WatcherOnDeleted(object sender, FileSystemEventArgs e)
        {
            Dispatcher.UIThread.Post(LoadKeys);
        }

        private void WatcherOnCreated(object sender, FileSystemEventArgs e)
        {
            Dispatcher.UIThread.Post(LoadKeys);
        }

        private void WatcherOnRenamed(object sender, RenamedEventArgs e)
        {
            Dispatcher.UIThread.Post(LoadKeys);
        }
    }
}