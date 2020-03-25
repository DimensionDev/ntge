use gumdrop::Options;

#[derive(Debug, Options)]
struct NtgeOptions {
    #[options(free, help = "Path to a file to read from.")]
    input: Option<String>,

    #[options(help = "Print this help message and exit.")]
    help: bool,

    #[options(help = "Print version info and exit.", short = "V")]
    version: bool,

    #[options(help = "Decrypt the input.")]
    decrypt: bool,

    #[options(help = "Encrypt to the specified RECIPIENT. May be repeated.")]
    recipient: Vec<String>,

    #[options(help = "Write the result to the file at path OUTPUT.")]
    output: Option<String>,
}

fn main() {
    let opts = NtgeOptions::parse_args_default_or_exit();

    if opts.version {
        println!("ntge {}", env!("CARGO_PKG_VERSION"));
        return;
    }
}
