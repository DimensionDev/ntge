//
//  Ed25519+PrivateKey.swift
//  NtgeCore
//
//  Created by Cirno MainasuK on 2020-4-15.
//

import Foundation

extension X25519 {

    public class PrivateKey: RustObject {

        var raw: OpaquePointer

        required init(raw: OpaquePointer) {
            self.raw = raw
        }

        func intoRaw() -> OpaquePointer {
            return raw
        }

        deinit {
            c_x25519_private_key_destroy(raw)
        }

    }

}
