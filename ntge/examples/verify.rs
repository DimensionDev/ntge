use std::io::prelude::*;
use std::process::{Command, Stdio};

static TESTSTRING: &'static str = "Decrypt Succeeds! Welcome to use NTGE!";

fn main() {
    let mut cmd = Command::new("./target/Debug/ntge");
    let output = match cmd
        .arg("encrypt")
        .args(&["-r", "id_ntge"])
        .args(&["-i", "id_ntge"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(_) => panic!("couldn't spawn"),
        Ok(output) => output,
    };

    match output.stdin.unwrap().write_all(TESTSTRING.as_bytes()) {
        Err(_) => panic!("couldn't write to stdin"),
        Ok(_) => (),
    }

    let mut cmd2 = Command::new("./target/Debug/ntge");
    cmd2.arg("verify")
        .args(&["-i", "id_ntge"])
        .stdin(output.stdout.unwrap())
        .spawn()
        .expect("You need to create a keypair with ntge-keygen first!");
}
