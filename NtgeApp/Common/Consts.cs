using System;

namespace NtgeApp.Common
{
    public class Consts
    {
        public const string NtgeFolder = ".ntge";

        public static string HomeDir => Environment.GetFolderPath(Environment.SpecialFolder.UserProfile);
    }
}