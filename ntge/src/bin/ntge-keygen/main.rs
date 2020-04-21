use std::process::exit;
use clap::{App, Arg};
use dirs::home_dir;
use ntge_core::ed25519;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

const DEFAULT_SAVE_PATH: &str = ".ntge";
const DEFAULT_FILE_NAME: &str = "id_ntge";
const DEFAULT_PUBLIC_KEY_SUFFIX: &str = "pub";
const DEFAULT_PRIVATE_KEY_SUFFIX: &str = "";

fn create_file_and_write(p: PathBuf, content: String) -> std::io::Result<()> {
    let mut file = match File::create(&p) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error: can not create file {}", p.display());
            eprintln!("{}", e);
            return Err(e);
        }
    };
    if let Err(e) = writeln!(file, "{}", content) {
        eprintln!("Error: can not write file {}", p.display());
        eprintln!("{}", e);
        return Err(e);
    }
    Ok(())
}

fn main() {
    let matches = App::new("ntge-keygen")
        .about("Not That Good Encryption key generator")
        .version(clap::crate_version!())
        .arg(
            Arg::with_name("file name")
                .short("f")
                .long("filename")
                .takes_value(true)
                .help("Key's file name, default to id_ntge"),
        )
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .takes_value(true)
                .help("Custom key file path"),
        )
        .arg(
            Arg::with_name("console")
                .short("c")
                .long("console")
                .takes_value(false)
                .help("Print key to console only"),
        )
        .get_matches();

    let keypair = ed25519::create_keypair();
    let public_key_content = ed25519::serialize_public_key(&(keypair.public));
    let private_key_content = ed25519::serialize_private_key(&(keypair.secret));
    if matches.is_present("console") {
        println!("{}", public_key_content);
        println!("{}", private_key_content);
        return;
    }
    let output_file = if let Some(path) = matches.value_of("path") {
        Path::new(&path).to_path_buf()
    } else {
        let mut home = match home_dir() {
            Some(dir) => dir,
            None => {
                println!("Warning: can not find home dir, use local dir");
                Path::new(".").to_path_buf()
            }
        };
        home.push(Path::new(DEFAULT_SAVE_PATH));
        if !home.exists() {
            if let Err(e) = fs::create_dir_all(&home) {
                eprintln!("Can not create {}", home.display());
                eprintln!("{}", e);
                exit(1);
            }
        }
        let filename = match matches.value_of("file name") {
            Some(name) => name,
            None => DEFAULT_FILE_NAME,
        };
        home.join(Path::new(filename))
    };
    if output_file.exists() {
        eprintln!("Error: file {} already exists!", output_file.display());
        exit(1);
    }
    let public_key_file = output_file.with_extension(DEFAULT_PUBLIC_KEY_SUFFIX);
    let private_key_file = output_file.with_extension(DEFAULT_PRIVATE_KEY_SUFFIX);
    if let Err(_) = create_file_and_write(public_key_file, public_key_content) {
        exit(1);
    }
    if let Err(_) = create_file_and_write(private_key_file, private_key_content) {
        exit(1);
    }
}
