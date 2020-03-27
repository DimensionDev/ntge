use dirs::home_dir;
use gumdrop::Options;
use ntge_core::*;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const DEFAULT_SAVE_PATH: &str = ".cube";
const DEFAULT_FILE_NAME: &str = "id_tessercube";
const DEFAULT_PUBLIC_KEY_SUFFIX: &str = "pub";
const DEFAULT_PRIVATE_KEY_SUFFIX: &str = "";

#[derive(Debug, Options)]
struct NtgeOptions {
    #[options(help = "Print this help message and exit.")]
    help: bool,

    #[options(help = "Print version info and exit.", short = "V")]
    version: bool,

    #[options(help = "Key's file name, default to id_tessercube")]
    filename: Option<String>,

    #[options(help = "Save path, default to ~/.cube/")]
    path: Option<String>,

    #[options(help = "Print key to console only")]
    console: bool,
}

fn main() {
    let opts = NtgeOptions::parse_args_default_or_exit();
    if opts.version {
        println!("ntge-keygen {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    let keypair = create_ed25519_keypair();
    let public_key_content = serialize_public_key(&keypair);
    let private_key_content = serialize_private_key(&keypair);
    if opts.console {
        println!("Your public key is");
        println!("{}", public_key_content);
        println!("Your private key is");
        println!("{}", private_key_content);
        return;
    }
    let output_folder = if let Some(path) = opts.path {
        Path::new(&path).to_path_buf()
    } else {
       let home = match home_dir() {
            Some(dir) => dir,
            None => {
                println!("Warning: can not find home dir, use local dir");
                Path::new(".").to_path_buf()
            }
        };
        home.join(Path::new(DEFAULT_SAVE_PATH))
    };
    if !output_folder.exists() {
        if let Err(e) = fs::create_dir_all(&output_folder) {
            eprintln!("Error: can not create .cube folder");
            eprintln!("{}", e);
            return;
        }
    }
    let output_file = if let Some(filename) = opts.filename {
        output_folder.join(Path::new(&filename))
    } else {
        output_folder.join(Path::new(DEFAULT_FILE_NAME))
    };
    if output_file.exists() {
        eprintln!("Error: file {} already exists!", output_file.display());
        return;
    }
    let public_key_path = output_file.with_extension(DEFAULT_PUBLIC_KEY_SUFFIX);
    let private_key_path = output_file.with_extension(DEFAULT_PRIVATE_KEY_SUFFIX);
    let mut output_public_key = match File::create(&public_key_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!(
                "Error: can not create file {} for public key",
                public_key_path.display()
            );
            eprintln!("{}", e);
            return;
        }
    };
    let mut output_private_key = match File::create(&private_key_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!(
                "Error: can not create file {} for private key",
                private_key_path.display()
            );
            eprintln!("{}", e);
            return;
        }
    };
    if let Err(e) = writeln!(output_public_key, "{}", public_key_content) {
        eprintln!(
            "Error: can not write file {} for public key",
            public_key_path.display()
        );
        eprintln!("{}", e);
    }
    if let Err(e) = writeln!(output_private_key, "{}", private_key_content) {
        eprintln!(
            "Error: can not write file {} for private key",
            private_key_path.display()
        );
        eprintln!("{}", e);
    }
    println!("Successfully create ntge key at {}", output_file.display());
}
