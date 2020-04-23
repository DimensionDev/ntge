//
//  FileKey.swift
//  NtgeCore
//
//  Created by Cirno MainasuK on 2020-4-20.
//

import Foundation

extension X25519 {
    
    public class FileKey: RustObject {
        
        var raw: OpaquePointer
        
        required init(raw: OpaquePointer) {
            self.raw = raw
        }
        
        func intoRaw() -> OpaquePointer {
            return raw
        }
        
        deinit {
            c_x25519_file_key_destroy(raw)
        }
        
    }
    
}


