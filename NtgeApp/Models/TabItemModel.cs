using Avalonia.Media;

namespace NtgeApp.Models
{
    internal class TabItemModel
    {
        public TabItemModel(string title, GeometryDrawing? icon, object content)
        {
            Title = title;
            Icon = icon;
            Content = content;
        }

        public string Title { get; }
        public GeometryDrawing? Icon { get; }
        public object Content { get; }
    }
}