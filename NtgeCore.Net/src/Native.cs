using System;
using System.Runtime.InteropServices;

namespace NtgeCore.Net
{
    public class Native
    {
        const string LIB_NAME = "ntge_core";

        [DllImport(LIB_NAME)]
        public static extern void free_string(IntPtr ptr);

        [DllImport(LIB_NAME)]
        public static extern void destroyEd25519PublicKey(IntPtr ptr);

        [DllImport(LIB_NAME)]
        public static extern IntPtr deserializeEd25519PublicKey(string str);

        [DllImport(LIB_NAME)]
        public static extern StringHandle serializeEd25519PublicKey(IntPtr ptr);

        [DllImport(LIB_NAME)]
        public static extern IntPtr newEd25519PrivateKey();

        [DllImport(LIB_NAME)]
        public static extern void destroyEd25519PrivateKey(IntPtr ptr);

        [DllImport(LIB_NAME)]
        public static extern IntPtr getPublicKeyFromEd25519PrivateKey(IntPtr ptr);

        [DllImport(LIB_NAME)]
        public static extern StringHandle serializeEd25519PrivateKey(IntPtr ptr);

        [DllImport(LIB_NAME)]
        public static extern IntPtr deserializeEd25519PrivateKey(string ptr);

        [DllImport(LIB_NAME)]
        public static extern IntPtr newEd25519Keypair();

        [DllImport(LIB_NAME)]
        public static extern void destroyEd25519Keypair(IntPtr ptr);

        [DllImport(LIB_NAME)]
        public static extern IntPtr getPrivateKeyFromEd25519Keypair(IntPtr ptr);

        [DllImport(LIB_NAME)]
        public static extern IntPtr getPublicKeyFromEd25519Keypair(IntPtr ptr);

        [DllImport(LIB_NAME)]
        public static extern IntPtr getEd25519KeypairFromPrivateKey(IntPtr ptr);

        [DllImport(LIB_NAME)]
        public static extern void destroyX25519PrivateKey(IntPtr ptr);

        [DllImport(LIB_NAME)]
        public static extern void destroyX25519PublicKey(IntPtr ptr);

        [DllImport(LIB_NAME)]
        public static extern void destroyX25519FileKey(IntPtr ptr);

        [DllImport(LIB_NAME)]
        public static extern void destroyMessage(IntPtr ptr);

        [DllImport(LIB_NAME)]
        public static extern StringHandle serializeMessage(IntPtr ptr);

        [DllImport(LIB_NAME)]
        public static extern IntPtr deserializeMessage(string ptr);

        [DllImport(LIB_NAME)]
        public static extern void destroyMessageDecryptor(IntPtr ptr);

        [DllImport(LIB_NAME)]
        public static extern IntPtr newMessageDecryptor();

        [DllImport(LIB_NAME)]
        [return: MarshalAs(UnmanagedType.I1)]
        public static extern bool messageDecryptorVerifyMessageMac(IntPtr decryptor_ptr, IntPtr file_key_ptr);

        [DllImport(LIB_NAME)]
        public static extern IntPtr messageDecryptorDecryptFileKey(IntPtr decryptor_ptr, IntPtr private_key_ptr);

        [DllImport(LIB_NAME)]
        public static extern StringHandle messageDecryptorDecryptPayload(IntPtr decryptor_ptr, IntPtr file_key_ptr);

        [DllImport(LIB_NAME)]
        [return: MarshalAs(UnmanagedType.I1)]
        public static extern bool messageDecryptorVerifySignature(IntPtr decryptor_ptr, IntPtr public_key_ptr);

        [DllImport(LIB_NAME)]
        public static extern IntPtr ed25519PublicKeyToX25519(IntPtr ptr);

        [DllImport(LIB_NAME)]
        public static extern IntPtr ed25519PrivateKeyToX25519(IntPtr ptr);

        [DllImport(LIB_NAME)]
        public static extern IntPtr newArrayForX25519PublicKey();

        [DllImport(LIB_NAME)]
        public static extern void destroyArrayX25519PublicKey(IntPtr ptr);

        [DllImport(LIB_NAME)]
        public static extern void pushArrayX25519PublicKey(IntPtr array_ptr, IntPtr element_ptr);

        [DllImport(LIB_NAME)]
        public static extern IntPtr newMessageEncryptor(IntPtr array_ptr);
        
        [DllImport(LIB_NAME)]
        public static extern void destroyMessageEncryptor(IntPtr ptr);

        [DllImport(LIB_NAME)]
        public static extern IntPtr encryptPlaintext(string input, IntPtr encryptor_ptr, IntPtr signature_key_ptr);
    }
}