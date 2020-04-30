use std::process::Command;

fn main() {
    let mut cmd = Command::new("cargo");
    match cmd
        .args(&["run", "--bin", "ntge-keygen"])
        .args(&["--", "-f", "example_key"])
        .spawn()
    {
        Err(_) => panic!("couldn't spawn"),
        Ok(_) => println!("Creation Succeeds!"),
    }
}
