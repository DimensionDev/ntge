//
//  Array.swift
//  NtgeCore
//
//  Created by Cirno MainasuK on 2020-4-16.
//

import Foundation

class RustArray<T: RustObject> {
    
    var raw: OpaquePointer
    
    public required init(raw: OpaquePointer) {
        self.raw = raw
    }
    
    deinit {
        
    }
    
}



