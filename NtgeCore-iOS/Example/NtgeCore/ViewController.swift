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
        
        let keypair = c_new_ed25519_keypair();
        
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }

}

