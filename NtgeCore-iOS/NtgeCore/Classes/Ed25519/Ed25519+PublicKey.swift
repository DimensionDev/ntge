//
//  Ed25519+PublicKey.swift
//  NtgeCore
//
//  Created by Cirno MainasuK on 2020-4-15.
//

import Foundation

extension Ed25519 {
    
    public class PublicKey: RustObject {
        
        var raw: OpaquePointer
        
        required init(raw: OpaquePointer) {
            self.raw = raw
        }
        
        public func intoRaw() -> OpaquePointer {
            return raw
        }
        
        deinit {
            c_ed25519_public_key_destroy(raw)
        }
        
    }
    
}

extension Ed25519.PublicKey {
    
    public func serialize() -> String {
        return String(cString: c_ed25519_public_key_serialize(raw))
    }
    
    public static func deserialize(serialized text: String) -> Ed25519.PublicKey? {
        return text
            .withCString { cstring in c_ed25519_public_key_deserialize(cstring) }
            .flatMap { pointer in Ed25519.PublicKey(raw: pointer) }
    }
    
}
