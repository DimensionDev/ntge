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
    
    func testEncryptAndDecrypt_withoutSignature() throws {
        let keypair = Ed25519.Keypair()
        let Ed25519PrivateKey = keypair.privateKey
        let Ed25519PublicKey = keypair.publicKey
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
        
        let signatureShouldNotVerified = Message.Decryptor.verifySignature(for: message, use: Ed25519PublicKey)
        XCTAssertFalse(signatureShouldNotVerified)
    }
    
    func testEncryptAndDecrypt_withSignature() throws {
        let keypair = Ed25519.Keypair()
        let Ed25519PrivateKey = keypair.privateKey
        let Ed25519PublicKey = keypair.publicKey
        let x25519PrivateKey = Ed25519PrivateKey.toX25519()
        let x25519PublicKey = Ed25519PrivateKey.publicKey.toX25519()
        let encryptor = Message.Encryptor(publicKeys: [x25519PublicKey])
        
        let plaintext = "Hello, World!"
        let plaintextData = Data(plaintext.utf8)
        let message = encryptor.encrypt(plaintext: plaintextData, signatureKey: Ed25519PrivateKey)
        
        let decryptor = Message.Decryptor(message: message)
        let fileKey = decryptor.decryptFileKey(privateKey: x25519PrivateKey)
        XCTAssertNotNil(fileKey)
        let payload = decryptor.decryptPayload(fileKey: fileKey!)
        XCTAssertNotNil(payload)
        let decryptedString = String(data: payload!, encoding: .utf8)
        XCTAssertEqual(plaintext, decryptedString)
        
        let signatureVerified = Message.Decryptor.verifySignature(for: message, use: Ed25519PublicKey)
        XCTAssertTrue(signatureVerified)
        
        let signatureShouldNotVerified = Message.Decryptor.verifySignature(for: message, use: Ed25519.PrivateKey().publicKey)
        XCTAssertFalse(signatureShouldNotVerified)
    }
    
}
