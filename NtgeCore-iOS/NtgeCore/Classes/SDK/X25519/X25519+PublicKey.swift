//
//  X25519+PublicKey.swift
//  NtgeCore
//
//  Created by Cirno MainasuK on 2020-4-16.
//

import Foundation

extension X25519 {
    
    public class PublicKey: RustObject {
        
        var raw: OpaquePointer
        
        required init(raw: OpaquePointer) {
            self.raw = raw
        }
        
        public func intoRaw() -> OpaquePointer {
            return raw
        }
        
        deinit {
            c_x25519_public_key_destroy(raw)
        }
        
    }
    
}
