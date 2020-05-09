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
        public static KeySource Instance { get; } = new KeySource();
        private readonly FileSystemWatcher _watcher;
        public ObservableCollection<KeyModel> Items { get;  } = new ObservableCollection<KeyModel>();

        private KeySource()
        {
            var path = Path.Combine(Consts.HomeDir, Consts.NtgeFolder);
            var files = Directory.GetFiles(path);
            foreach (var file in files)
            {
                Items.Add(new KeyModel(file));
            }

            _watcher = new FileSystemWatcher();
            _watcher.NotifyFilter = NotifyFilters.FileName | NotifyFilters.CreationTime | NotifyFilters.LastWrite;
            _watcher.Path = path;
            _watcher.Renamed += WatcherOnRenamed;
            _watcher.Created += WatcherOnCreated;
            _watcher.Deleted += WatcherOnDeleted;
            _watcher.EnableRaisingEvents = true;
        }
        
        private void WatcherOnDeleted(object sender, FileSystemEventArgs e)
        {
            var item = Items.FirstOrDefault(it => it.Path == e.FullPath);
            if (item != null)
            {
                Dispatcher.UIThread.Post(() => Items.Remove(item));
            }
        }

        private void WatcherOnCreated(object sender, FileSystemEventArgs e)
        {
            Dispatcher.UIThread.Post(() => Items.Add(new KeyModel(e.FullPath)));
        }

        private void WatcherOnRenamed(object sender, RenamedEventArgs e)
        {
            var item = Items.FirstOrDefault(it => it.Path == e.OldFullPath);
            if (item != null)
            {
                Dispatcher.UIThread.Post(() => item.Path = e.FullPath);
            }
        }
    }
}