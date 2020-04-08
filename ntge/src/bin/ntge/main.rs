use clap::{App, Arg};

mod encrypt;
mod util;

fn main() {
    let matches = App::new("ntge")
        .about("Not That Good Encryption")
        .version(clap::crate_version!())
        .subcommand(
            // encrypt command
            App::new("encrypt")
                .about("Encrypt message to recipients")
                .arg(
                    Arg::with_name("path")
                        .short("p")
                        .long("path")
                        .takes_value(true)
                        .help("Sets the input file path"),
                )
                .arg(
                    Arg::with_name("recipient")
                        .short("r")
                        .long("recipent")
                        .required(true)
                        .takes_value(true)
                        .help("Sets message recipient. Accept path and name.")
                        .multiple(true),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .takes_value(true)
                        .help("Sets encrypt output path")
                        .multiple(true),
                ),
        )
        .subcommand(
            App::new("decrypt")
                .about("decrypt message use private key")
                .arg(
                    Arg::with_name("path")
                        .short("p")
                        .long("path")
                        .takes_value(true)
                        .help("Sets the input file path"),
                )
                .arg(
                    Arg::with_name("identity")
                        .short("i")
                        .long("identity")
                        .takes_value(true)
                        .help("private identity key use for decrypt message"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("encrypt", arg_matches) => {
            let arg_matches = arg_matches.unwrap();
            let recipients = encrypt::recipient::fetch_recipient(&arg_matches);
            let plaintext = util::read_input_bytes(&arg_matches);
            // TODO: encrypt plaintext
            let content = plaintext;
            util::write_to_output(&arg_matches, &content);
        }
        ("decrypt", arg_matches) => {
            println!("decrypt!!!");
        }
        (_, _) => {
            // do nothing
        }
    }
}
