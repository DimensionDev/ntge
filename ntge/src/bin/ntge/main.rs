use clap::{App, Arg};
use ntge_core::{ed25519, message};

mod decrypt;
mod encrypt;
mod signature;
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
                        .help("Sets message recipient. Accept path and name")
                        .multiple(true),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .takes_value(true)
                        .help("Sets encrypt output path")
                        .multiple(true),
                )
                .arg(
                    Arg::with_name("identity")
                        .short("i")
                        .long("identity")
                        .takes_value(true)
                        .help("Sets signing key. Accept path and name")
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
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .takes_value(true)
                        .help("decrypt result file path"),
                )
                .arg(
                    Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .takes_value(false)
                        .help("show more information from decrypt"),
                ),
        )
        .subcommand(
            App::new("verify")
                .about("verify message signature")
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
                        .help("public identity key use for verify message signature"),
                ),
        )
        .subcommand(
            App::new("dump")
                .about("dump infomation for message or key")
                .arg(
                    Arg::with_name("path")
                        .short("p")
                        .long("path")
                        .takes_value(true)
                        .help("Sets the message or key path"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("encrypt", arg_matches) => {
            let arg_matches = arg_matches.unwrap();
            let recipients = encrypt::recipient::fetch_recipient(&arg_matches);
            let plaintext = util::read_input_bytes(&arg_matches);
            let identity = signature::fetch_signer(&arg_matches);
            let message = encrypt::encrypt_message(&plaintext, &recipients, identity.as_ref());
            let content = match message.serialize_to_armor() {
                Ok(armor) => armor,
                Err(e) => {
                    eprintln!("error: can not serialize message to armor.\nreason: {}", e);
                    std::process::exit(1);
                }
            };
            util::write_to_output(&arg_matches, &content.as_bytes());
        }
        ("decrypt", arg_matches) => {
            let arg_matches = arg_matches.unwrap();
            let plaintext = util::read_input_str(&arg_matches);
            let result =
                match decrypt::decrypt_message(&plaintext, arg_matches.value_of("identity")) {
                    Ok(it) => it,
                    Err(e) => {
                        eprintln!("{}", e.message);
                        std::process::exit(1);
                    }
                };
            if arg_matches.is_present("verbose") && arg_matches.is_present("output") {
                println!("{:?}", result.key);
            }
            util::write_to_output(&arg_matches, &result.content);
        }
        ("verify", arg_matches) => {
            let arg_matches = arg_matches.unwrap();
            let plaintext = util::read_input_str(&arg_matches);
            let identities = signature::fetch_verifier(&arg_matches);
            let result = match signature::verify_message_signature(&plaintext, &identities) {
                Ok(it) => it,
                Err(e) => {
                    eprintln!("{}", e.message);
                    std::process::exit(1);
                }
            };
            println!("message signature verified by {}", result.name);
        }
        ("dump", arg_matches) => {
            let arg_matches = arg_matches.unwrap();
            let input_bytes = util::read_input_bytes(&arg_matches);
            let input_text = match String::from_utf8(input_bytes) {
                Ok(text) => text,
                Err(e) => {
                    eprintln!("error: can not read content from input.\nreason: {}", e);
                    std::process::exit(1);
                }
            };
            if let Ok(message) = message::Message::deserialize_from_armor(&input_text) {
                println!("{:?}", message);
            } else if let Ok(public_key) = ed25519::deserialize_public_key(&input_text) {
                println!("{:#?}", public_key);
            } else if let Ok(private_key) = ed25519::deserialize_private_key(&input_text) {
                println!("{:#?}", private_key);
            }
        }
        (_, _) => {
            // do nothing
        }
    }
}
