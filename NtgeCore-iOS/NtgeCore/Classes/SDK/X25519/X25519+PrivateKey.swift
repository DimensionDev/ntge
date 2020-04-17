//
//  Ed25519+PrivateKey.swift
//  NtgeCore
//
//  Created by Cirno MainasuK on 2020-4-15.
//

import Foundation

//extension X25519 {
//
//    public class PrivateKey: RustObject {
//
//        var raw: OpaquePointer
//
//        required init(raw: OpaquePointer) {
//            self.raw = raw
//        }
//
//        public func intoRaw() -> OpaquePointer {
//            return raw
//        }
//
//        deinit {
//            c_x25519_private_key_destroy(raw)
//        }
//
//    }
//
//}

//extension X.PrivateKey {
//
//    public var publicKey: Ed25519.PublicKey {
//        Ed25519.PublicKey(raw: c_ed25519_private_key_get_public_key(raw))
//    }
//
//}
//
//extension Ed25519.PrivateKey {
//
//    public func serialize() -> String {
//        return String(cString: c_ed25519_private_key_serialize(raw))
//    }
//
//    public static func deserialize(serialized text: String) -> Ed25519.PrivateKey? {
//        return text
//            .withCString { cstring in c_ed25519_private_key_deserialize(cstring) }
//            .flatMap { pointer in Ed25519.PrivateKey(raw: pointer) }
//    }
//
//}
