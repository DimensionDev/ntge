//
//  ViewController.swift
//  NtgeCore
//
//  Created by mainasuk on 04/13/2020.
//  Copyright (c) 2020 mainasuk. All rights reserved.
//

import UIKit
import NtgeCore

class ViewController: UIViewController {

    override func viewDidLoad() {
        super.viewDidLoad()
        
        let keypair = c_ed25519_keypair_new()
        defer {
            c_ed25519_keypair_destroy(keypair)
        }
        
        let publicKey = c_ed25519_keypair_get_public_key(keypair)
        defer {
            c_ed25519_public_key_destroy(publicKey)
        }
        
        let serializedPublicKey = c_ed25519_public_key_serialize(publicKey)!
        print(String(cString: serializedPublicKey))
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }

}

