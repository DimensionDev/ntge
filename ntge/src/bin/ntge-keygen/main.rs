use gumdrop::Options;

#[derive(Debug, Options)]
struct NtgeOptions {
    #[options(help = "Print this help message and exit.")]
    help: bool,

    #[options(help = "Print version info and exit.", short = "V")]
    version: bool,

    #[options(help = "Write the result to the file at path OUTPUT. Defaults to standard output.")]
    output: Option<String>,
}

fn main() {
    let opts = NtgeOptions::parse_args_default_or_exit();

    if opts.version {
        println!("ntge-keygen {}", env!("CARGO_PKG_VERSION"));
        return;
    }
}
