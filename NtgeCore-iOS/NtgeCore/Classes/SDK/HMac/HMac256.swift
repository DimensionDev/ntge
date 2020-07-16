//
//  HMac256.swift
//  NtgeCore
//
//  Created by Cirno MainasuK on 2020-6-30.
//

import Foundation

public class HMac256 {
    public static func calculate(using publicKey: Ed25519.PublicKey, data: Data) -> Data? {
        var dataInput = data
        let mac = dataInput.withUnsafeMutableBytes { (pointer: UnsafeMutableRawBufferPointer) -> Data? in
            let bufferPointer = pointer.bindMemory(to: UInt8.self)
            let buffer = Buffer(data: bufferPointer.baseAddress, len: UInt(data.count))
            
            let macBuffer = c_hmac_utils_hmac256_calculate_using(publicKey.intoRaw(), buffer)
            guard macBuffer.len > 0 else {
                return nil
            }
            
            defer {
                c_buffer_destroy(macBuffer)
            }
            return Data(bytes: macBuffer.data, count: Int(macBuffer.len))
        }
        
        return mac
    }
}
