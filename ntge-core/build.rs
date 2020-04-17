extern crate cbindgen;

use cbindgen::Language;
use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let builder = cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(Language::C)
        .with_tab_width(4);

    match builder.generate() {
        Ok(gen) => gen,
        Err(e) => match e {
            // Ignore syntax errors because those will be handled later on by cargo build.
            cbindgen::Error::ParseSyntaxError {
                crate_name: _,
                src_path: _,
                error: _,
            } => return,
            _ => panic!("{:?}", e),
        },
    }
    .write_to_file("./include/ntge-core.h");
}
