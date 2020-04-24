//
//  NtgeCoreTests+X25519.swift
//  NtgeCore-Unit-Tests
//
//  Created by Cirno MainasuK on 2020-4-16.
//

import XCTest
import NtgeCore

class NtgeCoreTests_X25519: XCTestCase {

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

}

extension NtgeCoreTests_X25519 {
    
    func testPublicKeyConvertFromEd25519() {
        let ed25519PublicKey = Ed25519.Keypair().publicKey
        _ = ed25519PublicKey.toX25519()
    }
    
}

extension NtgeCoreTests_X25519 {
    
    func testPerformance_Ed25519_Private_toX25519_x10000() throws {
        let ed25519 = Ed25519.Keypair().privateKey
        // x10000
        self.measure {
            for _ in 0..<10000 {
                _ = ed25519.toX25519()
            }
        }
    }
    
    func testPerformance_Ed25519_Public_toX25519_x10000() throws {
        let ed25519 = Ed25519.Keypair().publicKey
        // x10000
        self.measure {
            for _ in 0..<10000 {
                _ = ed25519.toX25519()
            }
        }
    }
    
}
