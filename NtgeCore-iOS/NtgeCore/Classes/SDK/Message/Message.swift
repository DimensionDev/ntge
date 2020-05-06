//
//  Message.swift
//  NtgeCore
//
//  Created by Cirno MainasuK on 2020-4-16.
//

import Foundation

public class Message: RustObject {
    
    let raw: OpaquePointer
    
    public required init(raw: OpaquePointer) {
        self.raw = raw
    }
    
    func intoRaw() -> OpaquePointer {
        return raw
    }
    
    deinit {
        c_message_destory(raw)
    }
    
}

extension Message {
    
    public func serialize_to_armor() throws -> String? {
        var armor: UnsafeMutablePointer<Int8>? = nil
        
        _ = c_message_serialize_to_armor(raw, &armor)
        defer {
            if armor != nil {
                c_strings_destroy_c_char(&armor)
            }
        }
        
        guard let text = armor else {
            return nil
        }
        
        return String(cString: text)
    }
    
}
