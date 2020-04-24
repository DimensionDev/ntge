use std::fs;
use std::io::Write;
use std::io::{self, Read};
use std::path::Path;

pub(crate) const DEFAULT_SAVE_PATH: &str = ".ntge";

// read input from stdin or path
pub(crate) fn read_input_bytes(arg_matches: &clap::ArgMatches) -> Vec<u8> {
    if let Some(path) = arg_matches.value_of("path") {
        // read from file
        let file_path = Path::new(path);
        let file_bytes = match fs::read(&file_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("error: can not read file {}.\nreason: {}", path, e);
                std::process::exit(1);
            }
        };
        file_bytes
    } else {
        // read from stdin
        let mut input = String::new();
        match io::stdin().read_to_string(&mut input) {
            Ok(_) => input.as_bytes().to_vec(),
            Err(e) => {
                eprintln!("error: can not read content from stdin.\nreason: {}", e);
                std::process::exit(1);
            }
        }
    }
}

// read input from stdin or path
pub(crate) fn read_input_str(arg_matches: &clap::ArgMatches) -> String {
    if let Some(path) = arg_matches.value_of("path") {
        // read from file
        let file_path = Path::new(path);
        let file_str = match fs::read_to_string(&file_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("error: can not read file {}.\nreason: {}", path, e);
                std::process::exit(1);
            }
        };
        file_str
    } else {
        // read from stdin
        let mut input = String::new();
        match io::stdin().read_to_string(&mut input) {
            Ok(_) => input.as_str().to_string(),
            Err(e) => {
                eprintln!("error: can not read content from stdin.\nreason: {}", e);
                std::process::exit(1);
            }
        }
    }
}

pub(crate) fn write_to_output(arg_matches: &clap::ArgMatches, content: &[u8]) {
    if let Some(path) = arg_matches.value_of("output") {
        // write to file
        let file_path = Path::new(path);
        let mut file = match fs::File::create(&file_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("error: can not create file at path {}\nreason: {}", path, e);
                std::process::exit(1);
            }
        };
        match file.write_all(content) {
            Ok(_) => {
                // add trailing line break
                let _ = file.write_all(b"\n");
            }
            Err(e) => {
                eprintln!("error: can not write to file {}\nreason: {}", path, e);
                std::process::exit(1);
            }
        };
    } else {
        // write to stdout
        match io::stdout().lock().write_all(content) {
            Ok(_) => {
                // add trailing line break
                let _ = io::stdout().lock().write_all(b"\n");
            }
            Err(e) => {
                eprintln!("error: can not write to stdout\nreason: {}", e);
                std::process::exit(1);
            }
        }
    }
}
