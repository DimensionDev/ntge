use std::process::Command;

fn main() {
    let output = Command::new("./target/debug/ntge")
        .arg("encrypt")
        .arg("-i id_ntge")
        .arg("-p ntge/examples/test")
        .output()
        .expect("failed to execute process");
    let haha = Command::new("ls").output().expect("haha");
    println!("{:?}", output.stdout);
    println!("{:?}", haha.stdout);
}
