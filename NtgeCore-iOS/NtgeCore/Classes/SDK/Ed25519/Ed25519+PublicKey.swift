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
        
        func intoRaw() -> OpaquePointer {
            return raw
        }
        
        deinit {
            c_ed25519_public_key_destroy(raw)
        }
        
    }
    
}

extension Ed25519.PublicKey {
    
    public func serialize() -> String {
        var text = c_ed25519_public_key_serialize(raw)
        defer {
            c_strings_destroy_c_char(&text)
        }
        
        return String(cString: text!)
    }
    
    public static func deserialize(serialized text: String) -> Ed25519.PublicKey? {
        return text
            .withCString { cstring in c_ed25519_public_key_deserialize(cstring) }
            .flatMap { pointer in Ed25519.PublicKey(raw: pointer) }
    }
    
    public var keyID: String {
        var text = c_ed25519_public_key_key_id(raw)
        defer {
            c_strings_destroy_c_char(&text)
        }
        
        return String(cString: text!)
    }
    
}

extension Ed25519.PublicKey {
    
    public var x25519: X25519.PublicKey {
        X25519.PublicKey(raw: c_key_utils_ed25519_public_key_to_x25519(raw))
    }
    
}
