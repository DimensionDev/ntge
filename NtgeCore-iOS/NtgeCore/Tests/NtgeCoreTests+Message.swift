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
        let x25519PrivateKey = Ed25519PrivateKey.x25519
        let x25519PublicKey = Ed25519PrivateKey.publicKey.x25519
        let encryptor = Message.Encryptor(publicKeys: [x25519PublicKey])
        
        let plaintext = "Hello, World!"
        let plaintextData = Data(plaintext.utf8)
        let message = encryptor.encrypt(plaintext: plaintextData)
        let armor = try? message.serialize()
        XCTAssertNotNil(armor)
        let deserializedMessage = Message.deserialize(from: armor ?? "")
        
        let decryptor = Message.Decryptor(message: deserializedMessage!)
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
        let x25519PrivateKey = Ed25519PrivateKey.x25519
        let x25519PublicKey = Ed25519PrivateKey.publicKey.x25519
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
        
        let extraPayload = decryptor.decryptExtra(fileKey: fileKey!)
        XCTAssertNil(extraPayload)
        
        let signatureVerified = Message.Decryptor.verifySignature(for: message, use: Ed25519PublicKey)
        XCTAssertTrue(signatureVerified)
        
        let signatureShouldNotVerified = Message.Decryptor.verifySignature(for: message, use: Ed25519.PrivateKey().publicKey)
        XCTAssertFalse(signatureShouldNotVerified)
    }
    
    func testEncryptAndDecrypt_withExtra_withoutSignature() throws {
        let keypair = Ed25519.Keypair()
        let Ed25519PrivateKey = keypair.privateKey
        let Ed25519PublicKey = keypair.publicKey
        let x25519PrivateKey = Ed25519PrivateKey.x25519
        let x25519PublicKey = Ed25519PrivateKey.publicKey.x25519
        let encryptor = Message.Encryptor(publicKeys: [x25519PublicKey])
        
        let plaintext = "Hello, World!"
        let plaintextData = Data(plaintext.utf8)
        let extraPlaintext = "Hello, Extra!"
        let extraPlaintextData = Data(extraPlaintext.utf8)
        let message = encryptor.encrypt(plaintext: plaintextData, extraPlaintext: extraPlaintextData)
        
        let decryptor = Message.Decryptor(message: message)
        let fileKey = decryptor.decryptFileKey(privateKey: x25519PrivateKey)
        XCTAssertNotNil(fileKey)
        let payload = decryptor.decryptPayload(fileKey: fileKey!)
        XCTAssertNotNil(payload)
        let decryptedString = String(data: payload!, encoding: .utf8)
        XCTAssertEqual(plaintext, decryptedString)

        let extraPayload = decryptor.decryptExtra(fileKey: fileKey!)
        XCTAssertNotNil(payload)
        let decryptedExtraString = String(data: extraPayload!, encoding: .utf8)
        XCTAssertEqual(extraPlaintext, decryptedExtraString)
        
        let signatureShouldNotVerified = Message.Decryptor.verifySignature(for: message, use: Ed25519PublicKey)
        XCTAssertFalse(signatureShouldNotVerified)
    }
    
}

extension NtgeCoreTests_Message {
    
    func testMessageTimestamp() throws {
        let encryptor = self.newEncryptor(recipientCount: 1)
        
        let lengthInBytes = Measurement(value: 1, unit: UnitInformationStorage.megabytes).converted(to: .bytes).value
        let plaintext = randomData(ofLength: Int(lengthInBytes))
        
        let message = encryptor.encrypt(plaintext: plaintext)
        
        let date = message.timestamp
        XCTAssertNotNil(date)
    }
    
}

extension NtgeCoreTests_Message {
    
    func testPerformance_encrypt_1MB_10Recipient() throws {
        let encryptor = self.newEncryptor(recipientCount: 10)

        let lengthInBytes = Measurement(value: 10, unit: UnitInformationStorage.megabytes).converted(to: .bytes).value
        let plaintext = randomData(ofLength: Int(lengthInBytes))
        
        // 1MB to 10 Recipient
        self.measure {
            _ = encryptor.encrypt(plaintext: plaintext)
        }
    }
    
    func testPerformance_encrypt_100MB_10Recipient() throws {
        let encryptor = self.newEncryptor(recipientCount: 10)
        
        let lengthInBytes = Measurement(value: 100, unit: UnitInformationStorage.megabytes).converted(to: .bytes).value
        let plaintext = randomData(ofLength: Int(lengthInBytes))
        
        // 1MB to 10 Recipient
        self.measure {
            _ = encryptor.encrypt(plaintext: plaintext)
        }
    }
    
    func testPerformance_encrypt_1000MB_10Recipient() throws {
        let encryptor = self.newEncryptor(recipientCount: 10)
        
        let lengthInBytes = Measurement(value: 1000, unit: UnitInformationStorage.megabytes).converted(to: .bytes).value
        let plaintext = randomData(ofLength: Int(lengthInBytes))
        
        // 1MB to 10 Recipient
        self.measure {
            _ = encryptor.encrypt(plaintext: plaintext)
        }
    }
    
    // Compare: https://github.com/str4d/rage/issues/57
    // ~7.69s
    func testPerformance_encrypt_2GB_1Recipient() throws {
        let encryptor = self.newEncryptor(recipientCount: 1)
        
        let lengthInBytes = Measurement(value: 2, unit: UnitInformationStorage.gigabytes).converted(to: .bytes).value
        let plaintext = randomData(ofLength: Int(lengthInBytes))
        
        // 1MB to 10 Recipient
        self.measure {
            autoreleasepool {
                _ = encryptor.encrypt(plaintext: plaintext)
            }
        }
    }
    
}

extension NtgeCoreTests_Message {
    
    private func newEncryptor(recipientCount: Int) -> Message.Encryptor {
        let recipientKeys = [0..<recipientCount].map { _ in Ed25519.Keypair().publicKey.x25519 }
        return Message.Encryptor(publicKeys: recipientKeys)
    }
    
    private func randomData(ofLength length: Int) -> Data {
        var bytes = [UInt8](repeating: 0, count: length)
        let status = SecRandomCopyBytes(kSecRandomDefault, length, &bytes)
        if status == errSecSuccess {
            return Data(bytes)
        }
        
        fatalError()
    }
    
}
