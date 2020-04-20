//
//  NtgeCoreTests+Message.swift
//  NtgeCore-Unit-Tests
//
//  Created by Cirno MainasuK on 2020-4-20.
//

import XCTest
import NtgeCore

class NtgeCoreTests_Message: XCTestCase {

    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

}

extension NtgeCoreTests_Message {
    
    func testSmoke() { }
    
    func testEncryptor() throws {
        let x25519PublicKey = Ed25519.PrivateKey().publicKey.toX25519()
        let encryptor = Message.Encryptor(publicKeys: [x25519PublicKey])
        
        let plaintext = "Hello, World!"
        let plaintextData = Data(plaintext.utf8)
        let message = encryptor.encrypt(plaintext: plaintextData)
        let armor = try message.serialize_to_armor()
        
        XCTAssertNotNil(armor)
        XCTAssertTrue(armor!.hasPrefix("MsgBegin_"))
        XCTAssertTrue(armor!.hasSuffix("_EndMsg"))
    }
    
    func testDecryptor() throws {
        let Ed25519PrivateKey = Ed25519.PrivateKey()
        let x25519PrivateKey = Ed25519PrivateKey.toX25519()
        let x25519PublicKey = Ed25519PrivateKey.publicKey.toX25519()
        let encryptor = Message.Encryptor(publicKeys: [x25519PublicKey])
        
        let plaintext = "Hello, World!"
        let plaintextData = Data(plaintext.utf8)
        let message = encryptor.encrypt(plaintext: plaintextData)
        
        let decryptor = Message.Decryptor(message: message)
        let fileKey = decryptor.decryptFileKey(privateKey: x25519PrivateKey)
        XCTAssertNotNil(fileKey)
        let payload = decryptor.decryptPayload(fileKey: fileKey!)
        XCTAssertNotNil(payload)
        let decryptedString = String(data: payload!, encoding: .utf8)
        XCTAssertEqual(plaintext, decryptedString)
    }
    
}
