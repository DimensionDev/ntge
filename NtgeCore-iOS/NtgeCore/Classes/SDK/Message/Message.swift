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
    
    public func serialize() throws -> String? {
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
    
    public static func deserialize(from armor: String) -> Message? {
        return armor
            .withCString { cstring in c_message_deserialize_from_armor(cstring) }
            .flatMap { pointer in Message(raw: pointer) }
    }
    
}
