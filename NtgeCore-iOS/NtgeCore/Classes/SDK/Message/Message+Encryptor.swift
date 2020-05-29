//
//  Message+Encryptor.swift
//  NtgeCore
//
//  Created by Cirno MainasuK on 2020-4-16.
//

import Foundation

extension Message {
    
    public class Encryptor: RustObject {
        var raw: OpaquePointer
    
        required init(raw: OpaquePointer) {
            self.raw = raw
        }
        
        public convenience init(publicKeys keys: [X25519.PublicKey]) {
            let publicKeyArray = RustArray<X25519.PublicKey>(raw: c_array_new_for_x25519_public_key())
            for key in keys {
                publicKeyArray.append(publicKey: key)
            }
            
            let encryptor = c_message_encryptor_new(publicKeyArray.raw)!
            self.init(raw: encryptor)
            
            // destory array
            c_array_destroy_x25519_public_key(publicKeyArray.raw)
        }
        
        func intoRaw() -> OpaquePointer {
            return raw
        }
        
        deinit {
            c_message_encryptor_destroy(raw)
        }
    }
    
}

extension Message.Encryptor {
    
    public func encrypt(plaintext: Data, extraPlaintext: Data? = nil, signatureKey: Ed25519.PrivateKey? = nil) -> Message {
        var plaintextData = plaintext
        let message = plaintextData.withUnsafeMutableBytes { (pointer: UnsafeMutableRawBufferPointer) -> Message in
            let bufferPointer = pointer.bindMemory(to: UInt8.self)
            let buffer = Buffer(data: bufferPointer.baseAddress, len: UInt(plaintext.count))
            
            if let extraPlaintext = extraPlaintext {
                // with extra
                var extraPlaintextData = extraPlaintext
                let message = extraPlaintextData.withUnsafeMutableBytes { (extraPointer: UnsafeMutableRawBufferPointer) -> Message in
                    let extraBufferPointer = extraPointer.bindMemory(to: UInt8.self)
                    let extraBuffer = Buffer(data: extraBufferPointer.baseAddress, len: UInt(extraPlaintext.count))
                    return Message(raw: c_message_encryptor_encrypt_plaintext_and_extra(raw, buffer, extraBuffer, signatureKey?.intoRaw()))
                }
                
                return message
                
            } else {
                // no extra
                return Message(raw: c_message_encryptor_encrypt_plaintext(raw, buffer, signatureKey?.intoRaw()))
            }
        }   // pointer will dealloc here
        
        return message
    }
    
}

fileprivate extension RustArray {
    
    func append(publicKey: X25519.PublicKey) {
        c_array_push_x25519_public_key(raw, publicKey.raw)
    }
    
}
