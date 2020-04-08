use dirs::home_dir;
use gumdrop::Options;
use ntge_core::ed25519;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{Result, Write};
use std::path::{Path, PathBuf};

const DEFAULT_SAVE_PATH: &str = ".ntge";
const DEFAULT_FILE_NAME: &str = "id_ntge";
const DEFAULT_PUBLIC_KEY_SUFFIX: &str = "pub";
const DEFAULT_PRIVATE_KEY_SUFFIX: &str = "";

#[derive(Debug, Options)]
struct NtgeOptions {
    #[options(help = "Key's file name, default to id_ntge")]
    filename: Option<String>,

    #[options(help = "Save path, default to ~/.ntge/")]
    path: Option<String>,

    #[options(help = "Print key to console only")]
    console: bool,

    #[options(help = "Print version info and exit.", short = "v")]
    version: bool,

    #[options(help = "Print this help message and exit.")]
    help: bool,
}

fn create_file_and_write(p: PathBuf, content: String) -> Result<()> {
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
    let opts = NtgeOptions::parse_args_default_or_exit();
    if opts.version {
        println!("ntge-keygen {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    let keypair = ed25519::create_keypair();
    let public_key_content = ed25519::serialize_public_key(&(keypair.public));
    let private_key_content = ed25519::serialize_private_key(&(keypair.secret));
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
            eprintln!(
                "Error: can not create folder at {}",
                output_folder.display()
            );
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
    let public_key_file = output_file.with_extension(DEFAULT_PUBLIC_KEY_SUFFIX);
    let private_key_file = output_file.with_extension(DEFAULT_PRIVATE_KEY_SUFFIX);
    if let Err(_) = create_file_and_write(public_key_file, public_key_content) {
        return;
    }
    if let Err(_) = create_file_and_write(private_key_file, private_key_content) {
        return;
    }
    println!("Successfully create ntge key at {}", output_file.display());
}
