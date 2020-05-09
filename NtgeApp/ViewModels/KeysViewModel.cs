using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.IO;
using System.Linq;
using Avalonia.Threading;
using NtgeApp.Common;
using NtgeApp.DataSource;
using NtgeApp.Models;

namespace NtgeApp.ViewModels
{
    public class KeysViewModel : ViewModelBase
    {
        public IEnumerable<KeyModel> Items => KeySource.Instance.Items;
    }
}