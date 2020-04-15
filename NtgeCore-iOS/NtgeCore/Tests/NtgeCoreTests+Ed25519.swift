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

    func testExample() throws {
        // This is an example of a functional test case.
        // Use XCTAssert and related functions to verify your tests produce the correct results.
    }

    func testPerformanceExample() throws {
        // This is an example of a performance test case.
        self.measure {
            // Put the code you want to measure the time of here.
        }
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
        _ = Ed25519.Keypair(privateKey: privateKey)
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
    
}

