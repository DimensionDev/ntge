using System;

namespace NtgeApp.Common
{
    public class Consts
    {
        public static string HomeDir =>  (Environment.OSVersion.Platform == PlatformID.Unix || 
                                          Environment.OSVersion.Platform == PlatformID.MacOSX)
            ? Environment.GetEnvironmentVariable("HOME")
            : Environment.ExpandEnvironmentVariables("%HOMEDRIVE%%HOMEPATH%");

        public const string NtgeFolder = ".ntge";
    }
}