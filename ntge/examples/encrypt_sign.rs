use std::io::prelude::*;
use std::process::{Command, Stdio};

static TESTSTRING: &'static str = "Welcome to use NTGE!";

fn main() {
    let mut cmd = Command::new("cargo");
    let output = match cmd
        .args(&["run", "--bin", "ntge"])
        .arg("encrypt")
        .args(&["-r", "example_key"])
        .args(&["-i", "example_key"])
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
