//
//  NtgeCoreTests+HMac.swift
//  NtgeCore-Unit-Tests
//
//  Created by Cirno MainasuK on 2020-6-30.
//

import XCTest
import NtgeCore

class NtgeCoreTests_HMac: XCTestCase {

    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

}

extension NtgeCoreTests_HMac {
    
    func testSmoke() { }
    
    func testHMac256() {
        let publicKey = Ed25519.PublicKey.deserialize(serialized: "pub1ryd8qreac4s2tz0ect98sn5hpjc7254qu6ea748urn3u2mxygmfqtx0hvq-Ed25519")
        XCTAssertNotNil(publicKey)
        
        let input = Data("Hello, World!".utf8)
        let mac = HMac256.calculate(using: publicKey!, data: input)
        XCTAssertNotNil(mac)
        XCTAssertEqual(mac?.hexDescription, "45cf8a356f3cdebda7ccc08fdea82a7112f9ec14bae66f2e715e48ccd5ec2541")
        XCTAssertEqual(mac?.count ?? 0, 32)     // 32 bytes
        
        let randomPublicKey = Ed25519.Keypair().publicKey
        let randomMac = HMac256.calculate(using: randomPublicKey, data: Data())
        XCTAssertEqual(randomMac?.count ?? 0, 32)     // 32 bytes
        print(randomMac?.hexDescription ?? "<nil>")
    }
    
}

extension Data {
    var hexDescription: String {
        return reduce("") {$0 + String(format: "%02x", $1)}
    }
}
