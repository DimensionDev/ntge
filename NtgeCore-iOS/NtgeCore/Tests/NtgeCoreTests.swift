//
//  NtgeCoreTests.swift
//  NtgeCoreTests
//
//  Created by Cirno MainasuK on 2020-4-15.
//

import XCTest
import NtgeCore

class NtgeCoreTests: XCTestCase {
    
    override class func setUp() {
        
    }
    
    func testSmoke() { }

}

extension NtgeCoreTests {
    
    func testEncryptor() {
        let x25519PublicKey = Ed25519.PrivateKey().publicKey.toX25519()
        let encryptor = Message.Encryptor(publicKeys: [x25519PublicKey])
        let plaintext = "Hello, World!"
        let plaintextData = Data(plaintext.utf8)
        let message = encryptor.encrypt(plaintext: plaintextData)
    }
    
}

