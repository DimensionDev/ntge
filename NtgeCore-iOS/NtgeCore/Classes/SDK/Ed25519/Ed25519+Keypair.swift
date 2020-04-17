//
//  Ed25519+Keypair.swift
//  NtgeCore
//
//  Created by Cirno MainasuK on 2020-4-15.
//

import Foundation

extension Ed25519 {
    
    public class Keypair: RustObject {
        
        var raw: OpaquePointer
        
        required init(raw: OpaquePointer) {
            self.raw = raw
        }
        
        public convenience init() {
            self.init(raw: c_ed25519_keypair_new())
        }
        
        public convenience init(privateKey: Ed25519.PrivateKey) {
            self.init(raw: c_ed25519_keypair_construct_from_private_key(privateKey.intoRaw()))
        }
        
        public func intoRaw() -> OpaquePointer {
            return raw
        }
        
        deinit {
            c_ed25519_keypair_destroy(raw)
        }
        
    }
    
}

extension Ed25519.Keypair {
    
    public var privateKey: Ed25519.PrivateKey {
        Ed25519.PrivateKey(raw: c_ed25519_keypair_get_private_key(raw))
    }
    
    public var publicKey: Ed25519.PublicKey {
        Ed25519.PublicKey(raw: c_ed25519_keypair_get_public_key(raw))
    }
    
}
