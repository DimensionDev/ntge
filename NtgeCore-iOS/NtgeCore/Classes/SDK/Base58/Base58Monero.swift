//
//  Base58.swift
//  NtgeCore
//
//  Created by Cirno MainasuK on 2020-6-30.
//

import Foundation

public class Base58Monero {
    
    public static func encode(data: Data) -> String? {
        var inputData = data
        let encoded = inputData.withUnsafeMutableBytes { (pointer: UnsafeMutableRawBufferPointer) -> String? in
            let bufferPointer = pointer.bindMemory(to: UInt8.self)
            let buffer = Buffer(data: bufferPointer.baseAddress, len: UInt(data.count))
            
            var text: UnsafeMutablePointer<Int8>? = nil
            defer {
                if text != nil {
                    c_strings_destroy_c_char(&text)
                }
            }
            if c_base58_utils_encode(buffer, &text) == 0, let encodedText = text {
                return String(cString: encodedText)
            } else {
                return nil
            }
        }
        
        return encoded
    }
    
    public static func decode(string: String) -> Data? {
        string.withCString { cString in
            let buffer = c_base58_utils_decode(cString)
            guard buffer.len > 0 else {
                return nil
            }
            
            defer {
                c_buffer_destroy(buffer)
            }
            
            return Data(bytes: buffer.data, count: Int(buffer.len))
        }
    }
    
}
