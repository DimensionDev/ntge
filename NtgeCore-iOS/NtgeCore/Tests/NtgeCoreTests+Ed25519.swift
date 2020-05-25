//
//  NtgeCoreTests+Ed25519.swift
//  NtgeCore-Unit-Tests
//
//  Created by Cirno MainasuK on 2020-4-15.
//

import XCTest
import NtgeCore

class NtgeCoreTests_Ed25519: XCTestCase {

    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    func testSmoke() { }
    
}

extension NtgeCoreTests_Ed25519 {
    
    func testKeypair() {
        // test keypair and key access
        let keypair = Ed25519.Keypair()
        _ = keypair.privateKey
        _ = keypair.publicKey
    }
    
    func testKeypairFromPrivateKey() {
        // test construct keypair from private key
        let privateKey = Ed25519.PrivateKey()
        _ = Ed25519.Keypair(privateKey: privateKey)         // build keypair from private key
        _ = privateKey.serialize()                          // check private key availability
    }
    
    func testPrivateKey() {
        // test private key serialization
        let privateKey = Ed25519.PrivateKey()
        let serializedPrivateKey = privateKey.serialize()
        XCTAssertTrue(serializedPrivateKey.hasPrefix("pri"))
        XCTAssertTrue(serializedPrivateKey.hasSuffix("-Ed25519"))
        _ = Ed25519.PrivateKey.deserialize(serialized: serializedPrivateKey)
    }
    
    func testPublicKey() {
        // test public key creation
        let publicKey = Ed25519.PrivateKey().publicKey
        let serializedPublicKey = publicKey.serialize()
        XCTAssertTrue(serializedPublicKey.hasPrefix("pub"))
        XCTAssertTrue(serializedPublicKey.hasSuffix("-Ed25519"))
        _ = Ed25519.PublicKey.deserialize(serialized: serializedPublicKey)
    }
    
    func testPublicKey_keyID() {
        let publicKey = Ed25519.PrivateKey().publicKey
        let keyID = publicKey.keyID()
        let keyID2 = publicKey.keyID()
        print(keyID)
        XCTAssert(!keyID.isEmpty)
        XCTAssertEqual(keyID, keyID2)
    }
    
}

extension NtgeCoreTests_Ed25519 {
    
    func testPerformance_CreateKeypair_x1() throws {
        // x1
        self.measure {
            let _ = Ed25519.Keypair()
        }
    }
    
    func testPerformance_CreateKeypair_x100() throws {
        // x100
        self.measure {
            for _ in 0..<100 {
                let _ = Ed25519.Keypair()
            }
        }
    }
     
    func testPerformance_CreateKeypair_x1000() throws {
        // x1000
        self.measure {
            for _ in 0..<1000 {
                let _ = Ed25519.Keypair()
            }
        }
    }
    
    func testPerformance_CreateKeypair_x10000() throws {
        // x10000
        self.measure {
            for _ in 0..<10000 {
                let _ = Ed25519.Keypair()
            }
        }
    }
    
}

