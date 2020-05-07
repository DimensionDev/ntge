use cbindgen::{Builder, Language};
use std::env;

#[allow(dead_code)]
fn write_headers() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let builder = Builder::new()
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

fn main() {
    #[cfg(feature = "cbindgen-enable")]
    write_headers();
}
