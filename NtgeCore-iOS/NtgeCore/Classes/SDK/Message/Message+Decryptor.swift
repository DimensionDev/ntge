//
//  Message+Decryptor.swift
//  NtgeCore
//
//  Created by Cirno MainasuK on 2020-4-20.
//

import Foundation

extension Message {
    
    public class Decryptor: RustObject {
        
        var raw: OpaquePointer
        
        public required init(raw: OpaquePointer) {
            self.raw = raw
        }
                
        public convenience init(message: Message) {
            self.init(raw: c_message_decryptor_new(message.intoRaw())) 
        }
        
        func intoRaw() -> OpaquePointer {
            return raw
        }
        
        deinit {
            c_message_decryptor_destroy(raw)
        }
    }
    
}

extension Message.Decryptor {
    
    public func verifyMessageMac(fileKey: X25519.FileKey) -> Bool {
        return c_message_decryptor_verify_message_mac(raw, fileKey.intoRaw())
    }
    
    public func decryptFileKey(privateKey: X25519.PrivateKey) -> X25519.FileKey? {
        let fileKey = c_message_decryptor_decrypt_file_key(raw, privateKey.intoRaw()).flatMap { pointer in
            X25519.FileKey(raw: pointer)
        }
        
        return fileKey
    }
    
    public func decryptPayload(fileKey: X25519.FileKey) -> Data? {
        let buffer = c_message_decryptor_decrypt_payload(raw, fileKey.intoRaw())
        guard buffer.len > 0 else {
            return nil
        }
        
        defer {
            c_buffer_destroy(buffer)
        }
        return Data(bytes: buffer.data, count: Int(buffer.len))
    }
    
}
