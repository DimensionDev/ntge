use std::io::prelude::*;
use std::process::{Command, Stdio};

static TESTSTRING: &'static str = "Decrypt Succeeds! Welcome to use NTGE!";

fn main() {
    let mut cmd = Command::new("cargo");
    let output = match cmd
        .args(&["run", "--bin", "ntge"])
        .arg("encrypt")
        .args(&["-r", "example_key"])
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

    let mut cmd2 = Command::new("cargo");
    let _output = match cmd2
        .args(&["run", "--bin", "ntge"])
        .arg("decrypt")
        .args(&["-i", "example_key"])
        .stdin(output.stdout.unwrap())
        .spawn()
    {
        Err(_) => panic!("couldn't spawn"),
        Ok(output) => output,
    };
}
