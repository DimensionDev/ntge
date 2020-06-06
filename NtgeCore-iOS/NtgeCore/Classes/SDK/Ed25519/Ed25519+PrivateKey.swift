//
//  Ed25519+PrivateKey.swift
//  NtgeCore
//
//  Created by Cirno MainasuK on 2020-4-15.
//

import Foundation

extension Ed25519 {
    
    public class PrivateKey: RustObject {
        
        var raw: OpaquePointer
        
        required init(raw: OpaquePointer) {
            self.raw = raw
        }
        
        public convenience init() {
            self.init(raw: c_ed25519_private_key_new())
        }
        
        func intoRaw() -> OpaquePointer {
            return raw
        }
        
        deinit {
            c_ed25519_private_key_destroy(raw)
        }
        
    }
    
}

extension Ed25519.PrivateKey {
    
    public var publicKey: Ed25519.PublicKey {
        Ed25519.PublicKey(raw: c_ed25519_private_key_get_public_key(raw))
    }
    
}

extension Ed25519.PrivateKey {
    
    public func serialize() -> String {
        var text = c_ed25519_private_key_serialize(raw)
        defer {
            c_strings_destroy_c_char(&text)
        }
        return String(cString: text!)
    }
    
    public static func deserialize(from serialized: String) -> Ed25519.PrivateKey? {
        return serialized
            .withCString { cstring in c_ed25519_private_key_deserialize(cstring) }
            .flatMap { pointer in Ed25519.PrivateKey(raw: pointer) }
    }
    
}

extension Ed25519.PrivateKey {
    
    public var x25519: X25519.PrivateKey {
        X25519.PrivateKey(raw: c_key_utils_ed25519_private_key_to_x25519(raw))
    }
    
}
