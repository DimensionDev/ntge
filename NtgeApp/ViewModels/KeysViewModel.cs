using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.IO;
using NtgeApp.Common;
using NtgeApp.DataSource;
using NtgeApp.Models;

namespace NtgeApp.ViewModels
{
    public class KeysViewModel : ViewModelBase
    {
        public ObservableCollection<KeyModel> Items => KeySource.Instance.Items;
    }
}