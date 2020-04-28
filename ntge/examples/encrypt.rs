use std::io::prelude::*;
use std::process::{Command, Stdio};

static TESTSTRING: &'static str = "Decrypt Succeeds! Welcome to use NTGE!";

fn main() {
    let mut cmd = Command::new("./target/Debug/ntge");
    let output = match cmd
        .arg("encrypt")
        .args(&["-r", "id_ntge"])
        .stdin(Stdio::piped())
        .spawn()
    {
        Err(_) => panic!("couldn't spawn"),
        Ok(output) => output,
    };

    match output.stdin.unwrap().write_all(TESTSTRING.as_bytes()) {
        Err(_) => panic!("couldn't write to stdin"),
        Ok(_) => (),
    }
}
