//
//  NtgeCoreTests+Base58.swift
//  NtgeCore-Unit-Tests
//
//  Created by Cirno MainasuK on 2020-6-30.
//

import XCTest
import NtgeCore

class NtgeCoreTests_Base58: XCTestCase {

    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

}

extension NtgeCoreTests_Base58 {
    
    func testBase58Monero() {
        // encode data to text
        let input = Data("Hello, World!".utf8)
        let encoded = Base58Monero.encode(data: input)
        XCTAssertNotNil(encoded)
        XCTAssertEqual(encoded, "D7LMXYjYZ7cDaGe8bS")
        
        let decoded = Base58Monero.decode(string: encoded!)
        XCTAssertNotNil(decoded)
        XCTAssertEqual(input, decoded)
    }
    
}
