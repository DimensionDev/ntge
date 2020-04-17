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
    
    public func intoRaw() -> OpaquePointer {
        return raw
    }
    
    deinit {
        c_message_destory(raw)
    }
    
}

